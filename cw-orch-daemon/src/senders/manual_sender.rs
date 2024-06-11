use crate::proto::injective::InjectiveEthAccount;
use crate::queriers::Node;
use crate::tx_broadcaster::assert_broadcast_code_cosm_response;
use crate::{cosmos_modules, DaemonBase, TxBuilder};

use crate::{error::DaemonError, tx_resp::CosmTxResponse};

use cosmrs::proto::cosmos;
use cosmrs::proto::cosmos::auth::v1beta1::BaseAccount;
use cosmrs::proto::cosmos::vesting::v1beta1::PeriodicVestingAccount;
use cosmrs::tendermint::chain::Id;
use cosmrs::tx::{ModeInfo, Raw, SignDoc, SignMode, SignerInfo};
use cosmrs::{AccountId, Any};
use cosmwasm_std::Addr;
use cw_orch_core::environment::ChainInfoOwned;
use prost::Message;
use tonic::transport::Channel;

use std::io::{self, BufRead};

use super::sender_trait::SenderTrait;

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

impl SenderTrait for ManualSender {
    type Error = DaemonError;
    type SenderOptions = ManualSenderOptions;

    async fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        // We simulate
        let gas_needed = self.simulate(msgs, memo).await?;

        log::info!("Here is the transaction to sign and broadcast: ");
        log::info!("Gas needed: {}", gas_needed);

        log::info!("Enter the txhash to proceed");
        let stdin = io::stdin();
        let txhash = stdin.lock().lines().next().unwrap().unwrap();

        // We print the any messages to broadcast

        // We wait for the txhash as input to be able to continue the execution

        let resp = Node::new_async(self.grpc_channel())
            ._find_tx(txhash)
            .await?;

        assert_broadcast_code_cosm_response(resp)
    }

    fn address(&self) -> Result<Addr, DaemonError> {
        Ok(self.sender.clone())
    }

    fn msg_sender(&self) -> Result<AccountId, DaemonError> {
        self.sender.clone().to_string().parse().map_err(Into::into)
    }

    fn chain_info(&self) -> &cw_orch_core::environment::ChainInfoOwned {
        &self.chain_info
    }

    fn grpc_channel(&self) -> tonic::transport::Channel {
        self.grpc_channel.clone()
    }

    fn build(
        chain_info: cw_orch_core::environment::ChainInfoOwned,
        grpc_channel: tonic::transport::Channel,
        sender_options: Self::SenderOptions,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            chain_info,
            grpc_channel,
            sender: Addr::unchecked(
                sender_options
                    .sender_address
                    .expect("Manual sender needs an address"),
            ),
        })
    }

    fn set_options(&mut self, options: Self::SenderOptions) {
        self.sender = Addr::unchecked(
            options
                .sender_address
                .expect("Manual sender needs an address"),
        )
    }
}

impl ManualSender {
    pub async fn simulate(&self, msgs: Vec<Any>, memo: Option<&str>) -> Result<u64, DaemonError> {
        let timeout_height = Node::new_async(self.grpc_channel())._block_height().await? + 10u64;

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

        Node::new_async(self.grpc_channel())
            ._simulate_tx(tx_raw.to_bytes()?)
            .await
    }

    async fn base_account(&self) -> Result<BaseAccount, DaemonError> {
        let addr = self.address()?.to_string();

        let mut client = cosmos_modules::auth::query_client::QueryClient::new(self.grpc_channel());

        let resp = client
            .account(cosmos_modules::auth::QueryAccountRequest { address: addr })
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
