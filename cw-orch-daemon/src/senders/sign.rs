use crate::{
    queriers::Node,
    tx_broadcaster::{
        account_sequence_strategy, assert_broadcast_code_cosm_response, insufficient_fee_strategy,
        TxBroadcaster,
    },
    CosmTxResponse, DaemonError, QuerySender, TxBuilder, TxSender,
};
use cosmrs::{
    proto::cosmos::authz::v1beta1::MsgExec,
    tendermint::chain::Id,
    tx::{Body, Fee, Raw, SignDoc, SignerInfo},
    AccountId, Any,
};
use cosmwasm_std::Addr;
use prost::Message;

pub struct SigningAccount {
    pub account_number: u64,
    pub sequence: u64,
}

pub trait Signer: QuerySender<Error = DaemonError> + Sync {
    // --- General information about the signer --- //
    /// The chain id of the connected chain
    fn chain_id(&self) -> String;

    /// The account id of the signer.
    fn account_id(&self) -> AccountId;

    fn signing_account(
        &self,
    ) -> impl std::future::Future<Output = Result<SigningAccount, DaemonError>> + Send;

    /// Signals wether this signer is using authz
    /// If set to true, the signed messages will be wrapped inside authz messages
    fn authz_granter(&self) -> Option<&Addr> {
        None
    }

    // --- Related to transaction signing --- //
    /// Transaction signing
    fn sign(&self, sign_doc: SignDoc) -> Result<Raw, DaemonError>;

    fn signer_info(&self, sequence: u64) -> SignerInfo;

    fn build_fee(&self, amount: impl Into<u128>, gas_limit: u64) -> Result<Fee, DaemonError>;

    fn gas_price(&self) -> Result<f64, DaemonError>;

    /// Computes the gas needed for submitting a transaction
    fn calculate_gas(
        &self,
        tx_body: &Body,
        sequence: u64,
        account_number: u64,
    ) -> impl std::future::Future<Output = Result<u64, DaemonError>> + Send {
        async move {
            let fee = self.build_fee(0u8, 0)?;

            let auth_info = self.signer_info(sequence).auth_info(fee);

            let sign_doc = SignDoc::new(
                tx_body,
                &auth_info,
                &Id::try_from(self.chain_id())?,
                account_number,
            )?;

            let tx_raw = self.sign(sign_doc)?;

            Node::new_async(self.channel())
                ._simulate_tx(tx_raw.to_bytes()?)
                .await
        }
    }
}

impl<T: Signer + Sync> TxSender for T {
    fn account_id(&self) -> cosmrs::AccountId {
        self.account_id()
    }

    async fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let timeout_height = Node::new_async(self.channel())._block_height().await? + 10u64;

        let msgs = if self.authz_granter().is_some() {
            // We wrap authz messages
            vec![Any {
                type_url: "/cosmos.authz.v1beta1.MsgExec".to_string(),
                value: MsgExec {
                    grantee: self.account_id().to_string(),
                    msgs,
                }
                .encode_to_vec(),
            }]
        } else {
            msgs
        };

        let tx_body = TxBuilder::build_body(msgs, memo, timeout_height);

        let tx_builder = TxBuilder::new(tx_body);

        // We retry broadcasting the tx, with the following strategies
        // 1. In case there is an `incorrect account sequence` error, we can retry as much as possible (doesn't cost anything to the user)
        // 2. In case there is an insufficient_fee error, we retry once (costs fee to the user everytime we submit this kind of tx)
        // 3. In case there is an other error, we fail

        let tx_response = TxBroadcaster::default()
            .add_strategy(insufficient_fee_strategy())
            .add_strategy(account_sequence_strategy())
            .broadcast(tx_builder, self)
            .await?;

        let resp = Node::new_async(self.channel())
            ._find_tx(tx_response.txhash)
            .await?;

        assert_broadcast_code_cosm_response(resp)
    }
    /// Actual sender of the messages.
    /// This is different when using authz capabilites
    fn msg_sender(&self) -> Result<AccountId, DaemonError> {
        if let Some(sender) = self.authz_granter() {
            Ok(sender.as_str().parse()?)
        } else {
            Ok(self.account_id())
        }
    }
}
