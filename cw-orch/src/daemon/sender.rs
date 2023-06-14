use cosmrs::tx::{ModeInfo, SignMode};
use super::{
    chain_info::ChainKind,
    cosmos_modules::{self, auth::BaseAccount},
    error::DaemonError,
    queriers::{DaemonQuerier, Node},
    state::DaemonState,
    tx_resp::CosmTxResponse,
};
use crate::daemon::types::injective::InjectiveEthAccount;

use crate::{daemon::core::parse_cw_coins, keys::private::PrivateKey};
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

use cosmos_modules::vesting::PeriodicVestingAccount;
use tonic::transport::Channel;

const GAS_LIMIT: u64 = 1_000_000;
const GAS_BUFFER: f64 = 1.2;

/// A wallet is a sender of transactions, can be safely cloned and shared within the same thread.
pub type Wallet = Rc<Sender<All>>;

/// Signer of the transactions and helper for address derivation
/// This is the main interface for simulating and signing transactions
pub struct Sender<C: Signing + Context> {
    pub private_key: PrivateKey, // SigningKey
    pub secp: Secp256k1<C>,
    daemon_state: Rc<DaemonState>,
}

impl Sender<All> {
    pub fn new(daemon_state: &Rc<DaemonState>) -> Result<Sender<All>, DaemonError> {
        let kind = ChainKind::from(daemon_state.chain_data.network_type.clone());
        // NETWORK_MNEMONIC_GROUP
        let mnemonic = env::var(kind.mnemonic_name()).unwrap_or_else(|_| {
            panic!(
                "Wallet mnemonic environment variable {} not set.",
                kind.mnemonic_name()
            )
        });

        Self::from_mnemonic(daemon_state, &mnemonic)
    }

    /// Construct a new Sender from a mnemonic
    pub fn from_mnemonic(
        daemon_state: &Rc<DaemonState>,
        mnemonic: &str,
    ) -> Result<Sender<All>, DaemonError> {
        let secp = Secp256k1::new();
        let p_key: PrivateKey =
            PrivateKey::from_words(&secp, mnemonic, 0, 0, daemon_state.chain_data.slip44)?;

        let sender = Sender {
            daemon_state: daemon_state.clone(),
            private_key: p_key,
            secp,
        };
        log::info!(
            "Interacting with {} using address: {}",
            daemon_state.chain_data.chain_id,
            sender.pub_addr_str()?
        );
        Ok(sender)
    }

    fn cosmos_private_key(&self) -> SigningKey {
        SigningKey::from_slice(&self.private_key.raw_key()).unwrap()
    }

    pub fn channel(&self) -> Channel {
        self.daemon_state.grpc_channel.clone()
    }

    pub(crate) fn pub_addr(&self) -> Result<AccountId, DaemonError> {
        Ok(AccountId::new(
            &self.daemon_state.chain_data.bech32_prefix,
            &self.private_key.public_key(&self.secp).raw_address.unwrap(),
        )?)
    }

    pub fn address(&self) -> Result<Addr, DaemonError> {
        Ok(Addr::unchecked(self.pub_addr_str()?))
    }

    pub fn pub_addr_str(&self) -> Result<String, DaemonError> {
        Ok(self.pub_addr()?.to_string())
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
        let fee = Coin::new(
            amount.into(),
            &self.daemon_state.chain_data.fees.fee_tokens[0].denom,
        )
        .unwrap();
        let gas = gas_limit.unwrap_or(GAS_LIMIT);
        Fee::from_amount_and_gas(fee, gas)
    }

    pub async fn calculate_gas(
        &self,
        tx_body: &tx::Body,
        sequence: u64,
        account_number: u64,
    ) -> Result<u64, DaemonError> {
        let fee = self.build_fee(0u8, None);

        let auth_info =
            SignerInfo::single_direct(Some(self.cosmos_private_key().public_key()), sequence)
                .auth_info(fee);

        let sign_doc = SignDoc::new(
            tx_body,
            &auth_info,
            &Id::try_from(self.daemon_state.chain_data.chain_id.to_string())?,
            account_number,
        )?;

        let tx_raw = sign_doc.sign(&self.cosmos_private_key())?;

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
        let amount_to_pay = gas_expected
            * (self.daemon_state.chain_data.fees.fee_tokens[0].fixed_min_gas_price + 0.00001);

        log::debug!("Calculated gas needed: {:?}", amount_to_pay);

        let fee = self.build_fee(amount_to_pay as u128, Some(gas_expected as u64));

        let auth_info = SignerInfo {
            public_key: Some(self.private_key.get_signer_public_key(&self.secp)),
            mode_info: ModeInfo::single(SignMode::Direct),
            sequence,
        }
            .auth_info(fee);



        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &Id::try_from(self.daemon_state.chain_data.chain_id.to_string())?,
            account_number,
        )?;

        let tx_raw = sign_doc.sign(&self.cosmos_private_key())?;

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
        } else if let Ok(acc) = PeriodicVestingAccount::decode(account.as_ref()) {
            // try vesting account, (used by Terra2)
            acc.base_vesting_account.unwrap().base_account.unwrap()
        } else if let Ok(acc) = InjectiveEthAccount::decode(account.as_ref()) {
            acc.base_account.unwrap()
        } else {
            return Err(DaemonError::StdErr(
                "Unknown account type returned from QueryAccountRequest".into(),
            ));
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

        log::debug!("TX commit: {:?}", commit);

        let resp = Node::new(self.channel())
            .find_tx(commit.into_inner().tx_response.unwrap().txhash)
            .await?;

        // if tx result != 0 then the tx failed, so we return an error
        // if tx result == 0 then the tx succeeded, so we return the tx response
        if resp.code == 0 {
            Ok(resp)
        } else {
            Err(DaemonError::TxFailed {
                code: resp.code,
                reason: resp.raw_log,
            })
        }
    }
}
