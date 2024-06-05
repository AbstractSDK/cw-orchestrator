use std::str::FromStr;

use bitcoin::secp256k1::All;
use cosmrs::tx::{ModeInfo, SignMode};
use cosmrs::AccountId;
use cosmrs::{
    proto::cosmos::auth::v1beta1::BaseAccount,
    tendermint::chain::Id,
    tx::{self, Body, Fee, Raw, SequenceNumber, SignDoc, SignerInfo},
    Any, Coin,
};
use cw_orch_core::log::transaction_target;

use crate::sender::SenderOptions;

use super::{sender::Sender, DaemonError};

/// Struct used to build a raw transaction and broadcast it with a sender.
#[derive(Clone, Debug)]
pub struct TxBuilder {
    // # Required
    pub(crate) body: Body,
    // # Optional
    pub(crate) fee_amount: Option<u128>,
    pub(crate) gas_limit: Option<u64>,
    // if defined, use this sequence, else get it from the node
    pub(crate) sequence: Option<SequenceNumber>,
}

impl TxBuilder {
    /// Create a new TxBuilder with a given body.
    pub fn new(body: Body) -> Self {
        Self {
            body,
            fee_amount: None,
            gas_limit: None,
            sequence: None,
        }
    }
    /// Set a fixed fee amount for the tx
    pub fn fee_amount(&mut self, fee_amount: u128) -> &mut Self {
        self.fee_amount = Some(fee_amount);
        self
    }
    /// Set a gas limit for the tx
    pub fn gas_limit(&mut self, gas_limit: u64) -> &mut Self {
        self.gas_limit = Some(gas_limit);
        self
    }
    /// Set a sequence number for the tx
    pub fn sequence(&mut self, sequence: u64) -> &mut Self {
        self.sequence = Some(sequence);
        self
    }

    /// Builds the body of the tx with a given memo and timeout.
    pub fn build_body(msgs: Vec<Any>, memo: Option<&str>, timeout: u64) -> tx::Body {
        tx::Body::new(
            msgs,
            memo.unwrap_or("Tx committed using cw-orchestrator! ⚙️"),
            timeout as u32,
        )
    }

    pub(crate) fn build_fee(
        amount: impl Into<u128>,
        denom: &str,
        gas_limit: u64,
        sender_options: SenderOptions,
    ) -> Result<Fee, DaemonError> {
        let fee = Coin::new(amount.into(), denom).unwrap();
        let mut fee = Fee::from_amount_and_gas(fee, gas_limit);
        fee.granter = sender_options
            .fee_granter
            .map(|g| AccountId::from_str(&g))
            .transpose()?;
        Ok(fee)
    }

    /// Simulates the transaction and returns the necessary gas fee returned by the simulation on a node
    pub async fn simulate(&self, wallet: &Sender<All>) -> Result<u64, DaemonError> {
        // get the account number of the wallet
        let BaseAccount {
            account_number,
            sequence,
            ..
        } = wallet.base_account().await?;

        // overwrite sequence if set (can be used for concurrent txs)
        let sequence = self.sequence.unwrap_or(sequence);

        wallet
            .calculate_gas(&self.body, sequence, account_number)
            .await
    }

    /// Builds the raw tx with a given body and fee and signs it.
    /// Sets the TxBuilder's gas limit to its simulated amount for later use.
    pub async fn build(&mut self, wallet: &Sender<All>) -> Result<Raw, DaemonError> {
        // get the account number of the wallet
        let BaseAccount {
            account_number,
            sequence,
            ..
        } = wallet.base_account().await?;

        // overwrite sequence if set (can be used for concurrent txs)
        let sequence = self.sequence.unwrap_or(sequence);

        //
        let (tx_fee, gas_limit) = if let (Some(fee), Some(gas_limit)) =
            (self.fee_amount, self.gas_limit)
        {
            log::debug!(
                target: &transaction_target(),
                "Using pre-defined fee and gas limits: {}, {}",
                fee,
                gas_limit
            );
            (fee, gas_limit)
        } else {
            let sim_gas_used = wallet
                .calculate_gas(&self.body, sequence, account_number)
                .await?;
            log::debug!(target: &transaction_target(), "Simulated gas needed {:?}", sim_gas_used);

            let (gas_expected, fee_amount) = wallet.get_fee_from_gas(sim_gas_used)?;

            log::debug!(target: &transaction_target(), "Calculated fee needed: {:?}", fee_amount);
            // set the gas limit of self for future txs
            // there's no way to change the tx_builder body so simulation gas should remain the same as well
            self.gas_limit = Some(gas_expected);

            (fee_amount, gas_expected)
        };

        let fee = Self::build_fee(
            tx_fee,
            &wallet.get_fee_token(),
            gas_limit,
            wallet.options.clone(),
        )?;

        log::debug!(
            target: &transaction_target(),
            "submitting TX: \n fee: {:?}\naccount_nr: {:?}\nsequence: {:?}",
            fee,
            account_number,
            sequence
        );

        let auth_info = SignerInfo {
            public_key: wallet.private_key.get_signer_public_key(&wallet.secp),
            mode_info: ModeInfo::single(SignMode::Direct),
            sequence,
        }
        .auth_info(fee);

        let sign_doc = SignDoc::new(
            &self.body,
            &auth_info,
            &Id::try_from(wallet.chain_info.chain_id.to_string())?,
            account_number,
        )?;
        wallet.sign(sign_doc).map_err(Into::into)
    }
}
