use cosmrs::tx::{ModeInfo, SignMode};
use cosmrs::{
    proto::cosmos::auth::v1beta1::BaseAccount,
    tendermint::chain::Id,
    tx::{self, Body, Fee, Msg, Raw, SequenceNumber, SignDoc, SignerInfo},
    Any, Coin,
};
use secp256k1::All;

use super::{sender::Sender, DaemonError};

const GAS_BUFFER: f64 = 1.2;

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
    pub fn build_body<T: cosmrs::tx::Msg>(
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

    pub(crate) fn build_fee(amount: impl Into<u128>, denom: &str, gas_limit: u64) -> Fee {
        let fee = Coin::new(amount.into(), denom).unwrap();
        Fee::from_amount_and_gas(fee, gas_limit)
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
        let (tx_fee, gas_limit) =
            if let (Some(fee), Some(gas_limit)) = (self.fee_amount, self.gas_limit) {
                log::debug!(
                    "Using pre-defined fee and gas limits: {}, {}",
                    fee,
                    gas_limit
                );
                (fee, gas_limit)
            } else {
                let sim_gas_used = wallet
                    .calculate_gas(&self.body, sequence, account_number)
                    .await?;
                log::debug!("Simulated gas needed {:?}", sim_gas_used);

                let gas_expected = sim_gas_used as f64 * GAS_BUFFER;
                let fee_amount = gas_expected
                    * (wallet.daemon_state.chain_data.fees.fee_tokens[0].average_gas_price);

                log::debug!("Calculated fee needed: {:?}", fee_amount);
                // set the gas limit of self for future txs
                // there's no way to change the tx_builder body so simulation gas should remain the same as well
                self.gas_limit = Some(gas_expected as u64);

                (fee_amount as u128, gas_expected as u64)
            };

        let fee = Self::build_fee(
            tx_fee,
            &wallet.daemon_state.chain_data.fees.fee_tokens[0].denom,
            gas_limit,
        );

        log::debug!(
            "submitting tx: \n fee: {:?}\naccount_nr: {:?}\nsequence: {:?}",
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
            &Id::try_from(wallet.daemon_state.chain_data.chain_id.to_string())?,
            account_number,
        )?;
        wallet.sign(sign_doc).map_err(Into::into)
    }
}
