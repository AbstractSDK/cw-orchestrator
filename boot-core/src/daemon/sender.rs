use super::cosmos_modules::{self, auth::BaseAccount};
use super::queriers::DaemonQuerier;
use super::queriers::Node;
use super::{error::DaemonError, state::DaemonState, tx_resp::CosmTxResponse};
use crate::daemon::core::parse_cw_coins;
use crate::keys::private::PrivateKey;
use cosmrs::{
    bank::MsgSend,
    crypto::secp256k1::SigningKey,
    proto::traits::Message,
    tendermint::chain::Id,
    tx::{self, Fee, Msg, Raw, SignDoc, SignerInfo},
    AccountId, Any, Coin,
};
use cosmwasm_std::Addr;
use secp256k1::{All, Context, Secp256k1, Signing};
use std::{convert::TryFrom, env, rc::Rc, str::FromStr};

use tonic::transport::Channel;

const GAS_LIMIT: u64 = 1_000_000;
const GAS_BUFFER: f64 = 1.2;

pub type Wallet = Rc<Sender<All>>;

pub struct Sender<C: Signing + Context> {
    pub private_key: SigningKey,
    pub secp: Secp256k1<C>,
    daemon_state: Rc<DaemonState>,
}

impl Sender<All> {
    pub fn new(daemon_state: &Rc<DaemonState>) -> Result<Sender<All>, DaemonError> {
        let secp = Secp256k1::new();

        // NETWORK_MNEMONIC_GROUP
        let mnemonic = env::var(daemon_state.kind.mnemonic_name()).unwrap_or_else(|_| {
            panic!(
                "Wallet mnemonic environment variable {} not set.",
                daemon_state.kind.mnemonic_name()
            )
        });

        // use deployment mnemonic if specified, else use default network mnemonic
        let p_key: PrivateKey =
            PrivateKey::from_words(&secp, &mnemonic, 0, 0, daemon_state.chain.coin_type)?;

        let cosmos_private_key = SigningKey::from_bytes(&p_key.raw_key()).unwrap();

        let sender = Sender {
            daemon_state: daemon_state.clone(),
            private_key: cosmos_private_key,
            secp,
        };

        log::info!(
            "Interacting with {} using address: {}",
            daemon_state.chain_id,
            sender.pub_addr_str()?
        );

        Ok(sender)
    }

    pub fn channel(&self) -> Channel {
        self.daemon_state.grpc_channel.clone()
    }

    pub(crate) fn pub_addr(&self) -> Result<AccountId, DaemonError> {
        Ok(self
            .private_key
            .public_key()
            .account_id(&self.daemon_state.chain.pub_address_prefix)?)
    }

    pub fn address(&self) -> Result<Addr, DaemonError> {
        Ok(Addr::unchecked(
            self.private_key
                .public_key()
                .account_id(&self.daemon_state.chain.pub_address_prefix)?
                .to_string(),
        ))
    }

    pub fn pub_addr_str(&self) -> Result<String, DaemonError> {
        Ok(self
            .private_key
            .public_key()
            .account_id(&self.daemon_state.chain.pub_address_prefix)?
            .to_string())
    }

    pub async fn bank_send(
        &self,
        recipient: &str,
        coins: Vec<cosmwasm_std::Coin>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let msg_send = MsgSend {
            from_address: self.pub_addr()?,
            to_address: AccountId::from_str(recipient)?,
            amount: parse_cw_coins(&coins)?,
        };

        self.commit_tx(vec![msg_send], Some("sending tokens")).await
    }

    pub(crate) fn build_tx_body<T: Msg>(
        &self,
        msgs: Vec<T>,
        memo: Option<&str>,
        timeout: u64,
    ) -> tx::Body {
        let msgs = msgs
            .into_iter()
            .map(Msg::into_any)
            .collect::<Result<Vec<Any>, _>>()
            .unwrap();

        tx::Body::new(msgs, memo.unwrap_or_default(), timeout as u32)
    }

    pub(crate) fn build_fee(&self, amount: impl Into<u128>, gas_limit: Option<u64>) -> Fee {
        let amount = Coin {
            amount: amount.into(),
            denom: self.daemon_state.gas_denom.to_owned(),
        };

        let gas = gas_limit.unwrap_or(GAS_LIMIT);

        Fee::from_amount_and_gas(amount, gas)
    }

    pub async fn calculate_gas(
        &self,
        tx_body: &tx::Body,
        sequence: u64,
        account_number: u64,
    ) -> Result<u64, DaemonError> {
        let fee = self.build_fee(0u8, None);

        let auth_info =
            SignerInfo::single_direct(Some(self.private_key.public_key()), sequence).auth_info(fee);

        let sign_doc = SignDoc::new(
            tx_body,
            &auth_info,
            &Id::try_from(self.daemon_state.chain_id.clone())?,
            account_number,
        )?;

        let tx_raw = sign_doc.sign(&self.private_key)?;

        Node::new(self.channel())
            .simulate_tx(tx_raw.to_bytes()?)
            .await
    }

    pub async fn commit_tx<T: Msg>(
        &self,
        msgs: Vec<T>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let timeout_height = Node::new(self.channel()).block_height().await? + 10u64;

        let BaseAccount {
            account_number,
            sequence,
            ..
        } = self.base_account().await?;

        let tx_body = self.build_tx_body(msgs, memo, timeout_height);

        let sim_gas_used = self
            .calculate_gas(&tx_body, sequence, account_number)
            .await?;

        log::debug!("Simulated gas needed {:?}", sim_gas_used);

        let gas_expected = sim_gas_used as f64 * GAS_BUFFER;
        let amount_to_pay = gas_expected * (self.daemon_state.gas_price + 0.00001);

        log::debug!("Calculated gas needed: {:?}", amount_to_pay);

        let fee = self.build_fee(amount_to_pay as u128, Some(gas_expected as u64));

        let auth_info =
            SignerInfo::single_direct(Some(self.private_key.public_key()), sequence).auth_info(fee);

        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &Id::try_from(self.daemon_state.chain_id.clone())?,
            account_number,
        )?;

        let tx_raw = sign_doc.sign(&self.private_key)?;

        self.broadcast(tx_raw).await
    }

    // TODO: this does not work for injective because it's eth account
    pub async fn base_account(&self) -> Result<BaseAccount, DaemonError> {
        let addr = self.pub_addr().unwrap().to_string();

        let mut client = cosmos_modules::auth::query_client::QueryClient::new(self.channel());

        let resp = client
            .account(cosmos_modules::auth::QueryAccountRequest { address: addr })
            .await?
            .into_inner();

        log::debug!("base account query response: {:?}", resp);

        let account = resp.account.unwrap().value;

        let acc = if let Ok(acc) = BaseAccount::decode(account.as_ref()) {
            acc
        } else {
            // try vesting account, (used by Terra2)
            use cosmos_modules::vesting::PeriodicVestingAccount;

            let acc = PeriodicVestingAccount::decode(account.as_ref()).map_err(|_| {
                DaemonError::StdErr("Unknown account type returned from QueryAccountRequest".into())
            })?;

            acc.base_vesting_account.unwrap().base_account.unwrap()
        };

        Ok(acc)
    }

    async fn broadcast(&self, tx: Raw) -> Result<CosmTxResponse, DaemonError> {
        let mut client = cosmos_modules::tx::service_client::ServiceClient::new(self.channel());
        let commit = client
            .broadcast_tx(cosmos_modules::tx::BroadcastTxRequest {
                tx_bytes: tx.to_bytes()?,
                mode: cosmos_modules::tx::BroadcastMode::Sync.into(),
            })
            .await?;

        log::debug!("{:?}", commit);

        Node::new(self.channel())
            .find_tx_by_hash(commit.into_inner().tx_response.unwrap().txhash)
            .await
    }
}
