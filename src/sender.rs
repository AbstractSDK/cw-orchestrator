use cosmos_sdk_proto::{cosmos::{auth::v1beta1::BaseAccount}, tendermint::Protobuf};
use cosmrs::{
    bank::MsgSend,
    crypto::secp256k1::SigningKey,
    rpc::Client,
    tendermint::chain::Id,
    tx::{self, Fee, Gas, Msg, SignDoc, SignerInfo, Raw},
    AccountId, Any, Coin, Tx, dev,
};
use prost::Message;
use secp256k1::{All, Context, Secp256k1, Signing};
use serde_json::{from_reader, json, Map, Value};
use tokio::time::{timeout, sleep};
use std::{convert::TryFrom, env, fs::File, rc::Rc, str::FromStr, time::Duration};
use tonic::transport::Channel;

use crate::{
    config::GroupConfig,
    error::TerraRustScriptError,
    keys::private::PrivateKey,
    network::{Chain, NetworkConfig}, tx_resp::CosmTxResponse,
};

const GAS_LIMIT: u64 = 1_000_000;
const GAS_BUFFER: f64 = 1.2;

pub type Wallet<'a> = &'a Rc<Sender<All>>;

pub struct Sender<C: Signing + Context> {
    pub private_key: SigningKey,
    pub secp: Secp256k1<C>,
    network: NetworkConfig,
    channel: Channel,
}

impl<C: Signing + Context> Sender<C> {
    pub fn new(config: GroupConfig, secp: Secp256k1<C>) -> Result<Sender<C>, TerraRustScriptError> {
        // NETWORK_MNEMONIC_GROUP
        let mut composite_name = config.network_config.network.mnemonic_name().to_string();
        composite_name.push('_');
        composite_name.push_str(&config.name.to_ascii_uppercase());

        // use group mnemonic if specified, else use default network mnemonic
        let p_key: PrivateKey = if let Some(mnemonic) = env::var_os(&composite_name) {
            PrivateKey::from_words(
                &secp,
                mnemonic.to_str().unwrap(),
                0,
                0,
                config.network_config.chain.coin_type,
            )?
        } else {
            log::debug!("{}", config.network_config.network.mnemonic_name());
            let mnemonic = env::var(config.network_config.network.mnemonic_name())?;
            PrivateKey::from_words(
                &secp,
                &mnemonic,
                0,
                0,
                config.network_config.chain.coin_type,
            )?
        };

        let cosmos_private_key = SigningKey::from_bytes(&p_key.raw_key()).unwrap();

        Ok(Sender {
            // Cloning is encouraged: https://docs.rs/tonic/latest/tonic/transport/struct.Channel.html
            channel: config.network_config.grpc_channel.clone(),
            network: config.network_config,
            private_key: cosmos_private_key,
            secp,
        })
    }
    pub fn pub_addr(&self) -> Result<AccountId, TerraRustScriptError> {
        Ok(self
            .private_key
            .public_key()
            .account_id(&self.network.chain.pub_addr_prefix)?)
    }

    pub async fn bank_send(
        &self,
        recipient: &str,
        coins: Vec<Coin>,
    ) -> Result<CosmTxResponse, TerraRustScriptError> {
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
    ) -> Result<CosmTxResponse, TerraRustScriptError> {
        let timeout_height = 9001u16;
        let msgs: Result<Vec<Any>, _> = msgs.into_iter().map(Msg::into_any).collect();
        let msgs = msgs?;
        let gas_denom = self.network.gas_denom.clone();
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
            &Id::try_from(self.network.network_id.clone())?,
            account_number,
        )?;
        let tx_raw = sign_doc.sign(&self.private_key)?;

        let sim_gas_used = self.simulate_tx(tx_raw.to_bytes()?).await?;

        log::debug!("{:?}", sim_gas_used);

        let gas_expected = (sim_gas_used as f64 * GAS_BUFFER);
        let amount_to_pay = gas_expected * self.network.gas_price;
        let amount = Coin {
            amount: (amount_to_pay as u64).into(),
            denom: gas_denom,
        };
        let fee = Fee::from_amount_and_gas(amount, gas_expected as u64);


        let auth_info =
            SignerInfo::single_direct(Some(self.private_key.public_key()), sequence).auth_info(fee);
        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &Id::try_from(self.network.network_id.clone())?,
            account_number,
        )?;
        let tx_raw = sign_doc.sign(&self.private_key)?;

        self.broadcast(tx_raw).await
    }

    pub async fn base_account(&self) -> Result<BaseAccount, TerraRustScriptError> {
        let addr = self.pub_addr().unwrap().to_string();

        let mut client =
            cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient::new(self.channel());

        let resp = client
            .account(cosmos_sdk_proto::cosmos::auth::v1beta1::QueryAccountRequest { address: addr })
            .await?
            .into_inner();

        let acc: BaseAccount = BaseAccount::decode(resp.account.unwrap().value.as_ref()).unwrap();
        Ok(acc)
    }

    pub async fn simulate_tx(&self, tx_bytes: Vec<u8>) -> Result<u64, TerraRustScriptError> {
        let addr = self.pub_addr().unwrap().to_string();

        let mut client = cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient::new(
            self.channel(),
        );
        let resp = client
            .simulate(cosmos_sdk_proto::cosmos::tx::v1beta1::SimulateRequest {
                tx: None,
                tx_bytes: tx_bytes,
            })
            .await?
            .into_inner();

        let gas_used = resp.gas_info.unwrap().gas_used;
        Ok(gas_used)
    }

    fn channel(&self) -> Channel {
        self.channel.clone()
    }


    async fn broadcast(&self, tx: Raw) -> Result<CosmTxResponse, TerraRustScriptError>{
        let mut client = cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient::new(
            self.channel(),
        );

        let commit = client.broadcast_tx(cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest{
            tx_bytes: tx.to_bytes()?,
            mode: cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode::Async.into()
        }).await?;
        log::debug!("{:?}", commit);
        
        find_by_hash(&mut client, commit.into_inner().tx_response.unwrap().txhash ).await
    }

}

async fn find_by_hash(client: &mut cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient<Channel> ,hash: String) -> Result<CosmTxResponse, TerraRustScriptError>{
    let attempts = 7;
    let request = cosmos_sdk_proto::cosmos::tx::v1beta1::GetTxRequest{
        hash
    };
    for _ in 0..attempts {
        // TODO(tarcieri): handle not found errors
        if let Ok(tx) = client.get_tx(request.clone()).await {
            let resp = tx.into_inner().tx_response.unwrap();
            
            log::debug!("{:?}", resp);
            return Ok(resp.into())
        }
        sleep(Duration::from_secs(1)).await;
        // if let Ok(tx) =  {
        //     log::debug!("{:?}", tx);
        //     return tx;
        // }
    }
    panic!("couldn't find transaction after {} attempts!", attempts);
}