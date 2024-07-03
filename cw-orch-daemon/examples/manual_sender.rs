// This file illustrates an example for custom sender inside Daemon

use cw_orch_daemon::proto::injective::InjectiveEthAccount;
use cw_orch_daemon::queriers::Node;
use cw_orch_daemon::senders::builder::SenderBuilder;
use cw_orch_daemon::senders::query::QuerySender;
use cw_orch_daemon::tx_broadcaster::assert_broadcast_code_cosm_response;
use cw_orch_daemon::{DaemonBase, GrpcChannel, TxBuilder};

use cw_orch_daemon::{error::DaemonError, tx_resp::CosmTxResponse};

use cosmrs::proto::cosmos;
use cosmrs::proto::cosmos::auth::v1beta1::BaseAccount;
use cosmrs::proto::cosmos::vesting::v1beta1::PeriodicVestingAccount;
use cosmrs::tendermint::chain::Id;
use cosmrs::tx::{ModeInfo, Raw, SignDoc, SignMode, SignerInfo};
use cosmrs::{AccountId, Any};
use cosmwasm_std::Addr;
use cw_orch::prelude::*;
use cw_orch_core::environment::ChainInfoOwned;
use prost::Message;
use std::io::{self, Write};
use tonic::transport::Channel;

// ANCHOR: full_counter_example
use counter_contract::CounterContract;

// This is a test with a manual sender, to verify everything works, nothing is broadcasted

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok(); // Used to load the `.env` file if any
    pretty_env_logger::init(); // Used to log contract and chain interactions

    let network = cw_orch_networks::networks::JUNO_1;
    let sender = "juno1xjf5xscdk08c5es2m7epmerrpqmkmc3n98650t";
    let chain: ManualDaemon = ManualDaemon::builder(network).build_sender(ManualSenderOptions {
        sender_address: Some(sender.to_string()),
    })?;

    let counter = CounterContract::new(chain.clone());

    // Example tx hash that succeed (correspond to a code upload tx)
    // 58AA802705BEE4597A560FBC67F6C86400E66F5FCBD0F08AA37FB140BCD65B6D
    // If not found, try to find the latests juno code uploaded (4380+)
    // https://www.mintscan.io/juno/wasm/code/4380
    counter.upload()?;

    Ok(())
}

use cw_orch_daemon::senders::tx::TxSender;

pub type ManualDaemon = DaemonBase<ManualSender>;

#[derive(Clone, Default)]
pub struct ManualSenderOptions {
    pub sender_address: Option<String>,
}

/// Signer of the transactions and helper for address derivation
/// This is the main interface for simulating and signing transactions
#[derive(Clone)]
pub struct ManualSender {
    pub sender: Addr,
    pub chain_info: ChainInfoOwned,
    pub grpc_channel: Channel,
}

impl SenderBuilder for ManualSender {
    type Error = DaemonError;
    type Options = ManualSenderOptions;

    async fn build(
        chain_info: cw_orch_core::environment::ChainInfoOwned,
        sender_options: Self::Options,
    ) -> Result<Self, Self::Error> {
        let grpc_channel = GrpcChannel::from_chain_info(chain_info.clone()).await?;
        Ok(Self {
            chain_info,
            sender: Addr::unchecked(
                sender_options
                    .sender_address
                    .expect("Manual sender needs an address"),
            ),
            grpc_channel,
        })
    }
}

impl QuerySender for ManualSender {
    fn channel(&self) -> tonic::transport::Channel {
        self.grpc_channel.clone()
    }
}

impl TxSender for ManualSender {
    async fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        // We print the any messages to broadcast
        println!("Here is the transaction to sign and broadcast: ");
        println!("{:?}", msgs);
        // We simulate
        let gas_needed = self.simulate(msgs, memo).await?;
        println!("Gas needed: {}", gas_needed);

        // We wait for the txhash as input to be able to continue the execution
        println!("Enter the txhash to proceed");
        let mut txhash = String::new();
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed
        io::stdin()
            .read_line(&mut txhash)
            .expect("Failed to read line");
        let txhash = txhash.trim_end();

        let resp = Node::new_async(self.channel())
            ._find_tx(txhash.to_string())
            .await?;

        assert_broadcast_code_cosm_response(resp)
    }

    fn address(&self) -> Result<Addr, DaemonError> {
        Ok(self.sender.clone())
    }

    fn account_id(&self) -> Result<AccountId, DaemonError> {
        self.sender.clone().to_string().parse().map_err(Into::into)
    }
}

impl ManualSender {
    pub async fn simulate(&self, msgs: Vec<Any>, memo: Option<&str>) -> Result<u64, DaemonError> {
        let timeout_height = Node::new_async(self.channel())._block_height().await? + 10u64;

        let tx_body = TxBuilder::build_body(msgs, memo, timeout_height);

        let fee = TxBuilder::build_fee(0u8, &self.chain_info.gas_denom, 0, None)?;

        let BaseAccount {
            account_number,
            sequence,
            pub_key,
            ..
        } = self.base_account().await?;

        let auth_info = SignerInfo {
            public_key: pub_key.map(|key| key.try_into()).transpose()?,
            mode_info: ModeInfo::single(SignMode::Direct),
            sequence,
        }
        .auth_info(fee);

        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &Id::try_from(self.chain_info.chain_id.to_string())?,
            account_number,
        )?;

        let tx_raw: Raw = cosmos::tx::v1beta1::TxRaw {
            body_bytes: sign_doc.body_bytes,
            auth_info_bytes: sign_doc.auth_info_bytes,
            signatures: vec![vec![]],
        }
        .into();

        Node::new_async(self.channel())
            ._simulate_tx(tx_raw.to_bytes()?)
            .await
    }

    async fn base_account(&self) -> Result<BaseAccount, DaemonError> {
        let addr = self.address()?.to_string();

        let mut client =
            cosmrs::proto::cosmos::auth::v1beta1::query_client::QueryClient::new(self.channel());

        let resp = client
            .account(cosmrs::proto::cosmos::auth::v1beta1::QueryAccountRequest { address: addr })
            .await?
            .into_inner();

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
}
