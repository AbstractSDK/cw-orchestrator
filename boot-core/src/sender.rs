use crate::{
    cosmos_modules::{self, auth::BaseAccount},
    DaemonState,
};
use cosmrs::{
    bank::MsgSend,
    crypto::secp256k1::SigningKey,
    proto::{traits::Message},
    tendermint::chain::Id,
    tx::{self, Fee, Msg, Raw, SignDoc, SignerInfo},
    AccountId, Any, Coin,
};
use cosmwasm_std::Addr;
use secp256k1::{All, Context, Secp256k1, Signing};

use std::{convert::TryFrom, env, rc::Rc, str::FromStr, time::Duration};
use tokio::time::sleep;
use tonic::transport::Channel;

use crate::{error::BootError, keys::private::PrivateKey, CosmTxResponse};

const GAS_LIMIT: u64 = 1_000_000;
const GAS_BUFFER: f64 = 1.2;

pub type Wallet = Rc<Sender<All>>;

pub struct Sender<C: Signing + Context> {
    pub private_key: SigningKey,
    pub secp: Secp256k1<C>,
    daemon_state: Rc<DaemonState>,
}

impl Sender<All> {
    pub fn new(daemon_state: &Rc<DaemonState>) -> Result<Sender<All>, BootError> {
        let secp = Secp256k1::new();
        // NETWORK_MNEMONIC_GROUP
        let mnemonic = env::var(daemon_state.kind.mnemonic_name())?;

        // use deployment mnemonic if specified, else use default network mnemonic
        let p_key: PrivateKey =
            PrivateKey::from_words(&secp, &mnemonic, 0, 0, daemon_state.chain.coin_type)?;

        let cosmos_private_key = SigningKey::from_bytes(&p_key.raw_key()).unwrap();

        Ok(Sender {
            daemon_state: daemon_state.clone(),
            private_key: cosmos_private_key,
            secp,
        })
    }
    pub(crate) fn pub_addr(&self) -> Result<AccountId, BootError> {
        Ok(self
            .private_key
            .public_key()
            .account_id(self.daemon_state.chain.pub_address_prefix)?)
    }
    pub fn address(&self) -> Result<Addr, BootError> {
        Ok(Addr::unchecked(
            self.private_key
                .public_key()
                .account_id(self.daemon_state.chain.pub_address_prefix)?
                .to_string(),
        ))
    }

    pub fn pub_addr_str(&self) -> Result<String, BootError> {
        Ok(self
            .private_key
            .public_key()
            .account_id(self.daemon_state.chain.pub_address_prefix)?
            .to_string())
    }

    pub async fn bank_send(
        &self,
        recipient: &str,
        coins: Vec<Coin>,
    ) -> Result<CosmTxResponse, BootError> {
        let msg_send = MsgSend {
            from_address: self.pub_addr()?,
            to_address: AccountId::from_str(recipient)?,
            amount: coins,
        };

        self.commit_tx(vec![msg_send], Some("sending tokens")).await
    }

    pub async fn commit_tx<T: Msg>(
        &self,
        msgs: Vec<T>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, BootError> {
        let timeout_height = self.block_height().await? + 10u32;
        let msgs: Result<Vec<Any>, _> = msgs.into_iter().map(Msg::into_any).collect();
        let msgs = msgs?;
        let gas_denom = self.daemon_state.gas_denom.clone();
        let amount = Coin {
            amount: 0u8.into(),
            denom: gas_denom.clone(),
        };
        let fee = Fee::from_amount_and_gas(amount, GAS_LIMIT);

        let BaseAccount {
            account_number,
            sequence,
            ..
        } = self.base_account().await?;

        let tx_body = tx::Body::new(msgs, memo.unwrap_or_default(), timeout_height);
        let auth_info =
            SignerInfo::single_direct(Some(self.private_key.public_key()), sequence).auth_info(fee);
        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &Id::try_from(self.daemon_state.id.clone())?,
            account_number,
        )?;
        let tx_raw = sign_doc.sign(&self.private_key)?;

        let sim_gas_used = self.simulate_tx(tx_raw.to_bytes()?).await?;

        log::debug!("{:?}", sim_gas_used);

        let gas_expected = sim_gas_used as f64 * GAS_BUFFER;
        let amount_to_pay = gas_expected * self.daemon_state.gas_price;
        log::debug!("gas fee: {:?}", amount_to_pay);
        let amount = Coin {
            amount: (amount_to_pay as u64).into(),
            denom: gas_denom,
        };
        let fee = Fee::from_amount_and_gas(amount, gas_expected as u64);
        // log::debug!("{:?}", self.pub_addr_str());
        let auth_info =
            SignerInfo::single_direct(Some(self.private_key.public_key()), sequence).auth_info(fee);
        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &Id::try_from(self.daemon_state.id.clone())?,
            account_number,
        )?;
        let tx_raw = sign_doc.sign(&self.private_key)?;

        self.broadcast(tx_raw).await
    }

    pub async fn base_account(&self) -> Result<BaseAccount, BootError> {
        let addr = self.pub_addr().unwrap().to_string();

        let mut client = cosmos_modules::auth::query_client::QueryClient::new(self.channel());

        let resp = client
            .account(cosmos_modules::auth::QueryAccountRequest { address: addr })
            .await?
            .into_inner();

        let acc: BaseAccount = BaseAccount::decode(resp.account.unwrap().value.as_ref()).unwrap();
        Ok(acc)
    }

    pub async fn simulate_tx(&self, tx_bytes: Vec<u8>) -> Result<u64, BootError> {
        let _addr = self.pub_addr().unwrap().to_string();

        let mut client = cosmos_modules::tx::service_client::ServiceClient::new(self.channel());
        #[allow(deprecated)]
        let resp = client
            .simulate(cosmos_modules::tx::SimulateRequest { tx: None, tx_bytes })
            .await?
            .into_inner();
        let gas_used = resp.gas_info.unwrap().gas_used;
        Ok(gas_used)
    }

    pub fn channel(&self) -> Channel {
        self.daemon_state.grpc_channel.clone()
    }

    async fn block_height(&self) -> Result<u32, BootError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel());
        #[allow(deprecated)]
        let resp = client
            .get_latest_block(cosmos_modules::tendermint::GetLatestBlockRequest {})
            .await?
            .into_inner();
        Ok(resp.block.unwrap().header.unwrap().height as u32)
    }

    async fn broadcast(&self, tx: Raw) -> Result<CosmTxResponse, BootError> {
        let mut client = cosmos_modules::tx::service_client::ServiceClient::new(self.channel());

        let commit = client
            .broadcast_tx(cosmos_modules::tx::BroadcastTxRequest {
                tx_bytes: tx.to_bytes()?,
                mode: cosmos_modules::tx::BroadcastMode::Sync.into(),
            })
            .await?;
        log::debug!("{:?}", commit);

        find_by_hash(&mut client, commit.into_inner().tx_response.unwrap().txhash).await
    }
}

pub async fn find_by_hash(
    client: &mut cosmos_modules::tx::service_client::ServiceClient<Channel>,
    hash: String,
) -> Result<CosmTxResponse, BootError> {
    let attempts = 20;
    let request = cosmos_modules::tx::GetTxRequest { hash };
    for _ in 0..attempts {
        if let Ok(tx) = client.get_tx(request.clone()).await {
            let resp = tx.into_inner().tx_response.unwrap();

            log::debug!("{:?}", resp);
            return Ok(resp.into());
        }
        sleep(Duration::from_secs(10)).await;
    }
    panic!("couldn't find transaction after {} attempts!", attempts);
}
