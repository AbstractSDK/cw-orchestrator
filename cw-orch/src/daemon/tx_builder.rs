use cosmrs::{
    proto::cosmos::{auth::v1beta1::BaseAccount, tx::v1beta1::TxRaw},
    tendermint::chain::Id,
    tx::{self, Body, Fee, Msg, Raw, SequenceNumber, SignDoc, SignerInfo},
    Any, Coin,
};
use secp256k1::All;

use super::{sender::Sender, DaemonError, Wallet};

const GAS_BUFFER: f64 = 1.2;
const GAS_LIMIT: u64 = 200_000_000;

/// Struct used to build a raw transaction and broadcast it with a sender.
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
    pub fn new(body: Body) -> Self {
        Self {
            body,
            fee_amount: None,
            gas_limit: None,
            sequence: None,
        }
    }

    pub fn body(&mut self, body: Body) -> &mut Self {
        self.body = body;
        self
    }

    pub fn fee_amount(&mut self, fee_amount: u128) -> &mut Self {
        self.fee_amount = Some(fee_amount);
        self
    }

    pub fn gas_limit(&mut self, gas_limit: u64) -> &mut Self {
        self.gas_limit = Some(gas_limit);
        self
    }

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
    pub async fn build(&self, wallet: &Sender<All>) -> Result<Raw, DaemonError> {
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
            (fee, gas_limit)
        } else {
            let sim_gas_used = wallet
                .calculate_gas(&self.body, sequence, account_number)
                .await?;
            log::debug!("Simulated gas needed {:?}", sim_gas_used);

            let gas_expected = sim_gas_used as f64 * GAS_BUFFER;
            let fee_amount = gas_expected
                * (wallet.daemon_state.chain_data.fees.fee_tokens[0].fixed_min_gas_price + 0.00001);

            log::debug!("Calculated fee needed: {:?}", fee_amount);
            (fee_amount as u128, gas_expected as u64)
        };

        let fee = Self::build_fee(
            tx_fee,
            &wallet.daemon_state.chain_data.fees.fee_tokens[0].denom,
            gas_limit,
        );

        let auth_info = SignerInfo::single_direct(Some(wallet.private_key.public_key()), sequence)
            .auth_info(fee);

        let sign_doc = SignDoc::new(
            &self.body,
            &auth_info,
            &Id::try_from(wallet.daemon_state.chain_data.chain_id.to_string())?,
            account_number,
        )?;

        sign_doc.sign(&wallet.private_key).map_err(Into::into)
    }
}
