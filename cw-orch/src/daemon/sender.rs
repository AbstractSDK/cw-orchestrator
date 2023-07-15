use crate::daemon::proto::injective::ETHEREUM_COIN_TYPE;

use super::{
    chain_info::ChainKind,
    cosmos_modules::{self, auth::BaseAccount},
    error::DaemonError,
    queriers::{DaemonQuerier, Node},
    state::DaemonState,
    tx_builder::TxBuilder,
    tx_resp::CosmTxResponse,
};
use crate::daemon::proto::injective::InjectiveEthAccount;

#[cfg(feature = "eth")]
use crate::daemon::proto::injective::InjectiveSigner;

use crate::{daemon::core::parse_cw_coins, keys::private::PrivateKey};
use cosmrs::{
    bank::MsgSend,
    crypto::secp256k1::SigningKey,
    proto::traits::Message,
    tendermint::chain::Id,
    tx::{self, ModeInfo, Msg, Raw, SignDoc, SignMode, SignerInfo},
    AccountId,
};
use cosmwasm_std::Addr;
use secp256k1::{All, Context, Secp256k1, Signing};
use std::{convert::TryFrom, env, rc::Rc, str::FromStr};

use cosmos_modules::vesting::PeriodicVestingAccount;
use tonic::transport::Channel;

/// A wallet is a sender of transactions, can be safely cloned and shared within the same thread.
pub type Wallet = Rc<Sender<All>>;

/// Signer of the transactions and helper for address derivation
/// This is the main interface for simulating and signing transactions
pub struct Sender<C: Signing + Context> {
    pub private_key: PrivateKey,
    pub secp: Secp256k1<C>,
    pub(crate) daemon_state: Rc<DaemonState>,
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

    pub async fn calculate_gas(
        &self,
        tx_body: &tx::Body,
        sequence: u64,
        account_number: u64,
    ) -> Result<u64, DaemonError> {
        let fee = TxBuilder::build_fee(
            0u8,
            &self.daemon_state.chain_data.fees.fee_tokens[0].denom,
            0,
        );

        let auth_info = SignerInfo {
            public_key: self.private_key.get_signer_public_key(&self.secp),
            mode_info: ModeInfo::single(SignMode::Direct),
            sequence,
        }
        .auth_info(fee);

        let sign_doc = SignDoc::new(
            tx_body,
            &auth_info,
            &Id::try_from(self.daemon_state.chain_data.chain_id.to_string())?,
            account_number,
        )?;

        let tx_raw = self.sign(sign_doc)?;

        Node::new(self.channel())
            .simulate_tx(tx_raw.to_bytes()?)
            .await
    }

    pub async fn commit_tx<T: Msg>(
        &self,
        msgs: Vec<T>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let timeout_height = Node::new(self.channel()).block_height().await? + 50u64;

        let tx_body = TxBuilder::build_body(msgs, memo, timeout_height);

        let mut tx_builder = TxBuilder::new(tx_body);

        // now commit and check the result, if we get an insufficient fee error, we can try again with the proposed fee

        let tx = tx_builder.build(self).await?;

        let mut tx_response = self.broadcast_tx(tx).await?;

        log::debug!("tx broadcast response: {:?}", tx_response);

        if has_insufficient_fee(&tx_response.raw_log) {
            // get the suggested fee from the error message
            let suggested_fee = parse_suggested_fee(&tx_response.raw_log);

            let Some(new_fee) = suggested_fee else {
                return Err(DaemonError::InsufficientFee(
                    tx_response.raw_log,
                ));
            };

            // update the fee and try again
            tx_builder.fee_amount(new_fee);
            let tx = tx_builder.build(self).await?;

            tx_response = self.broadcast_tx(tx).await?;
        }

        let resp = Node::new(self.channel())
            .find_tx(tx_response.txhash)
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

    pub fn sign(&self, sign_doc: SignDoc) -> Result<Raw, DaemonError> {
        let tx_raw = if self.private_key.coin_type == ETHEREUM_COIN_TYPE {
            #[cfg(not(feature = "eth"))]
            panic!(
                "Coin Type {} not supported without eth feature",
                ETHEREUM_COIN_TYPE
            );
            #[cfg(feature = "eth")]
            self.private_key.sign_injective(sign_doc)?
        } else {
            sign_doc.sign(&self.cosmos_private_key())?
        };
        Ok(tx_raw)
    }

    pub async fn base_account(&self) -> Result<BaseAccount, DaemonError> {
        let addr = self.pub_addr().unwrap().to_string();

        let mut client = cosmos_modules::auth::query_client::QueryClient::new(self.channel());

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

    async fn broadcast_tx(
        &self,
        tx: Raw,
    ) -> Result<cosmrs::proto::cosmos::base::abci::v1beta1::TxResponse, DaemonError> {
        let mut client = cosmos_modules::tx::service_client::ServiceClient::new(self.channel());
        let commit = client
            .broadcast_tx(cosmos_modules::tx::BroadcastTxRequest {
                tx_bytes: tx.to_bytes()?,
                mode: cosmos_modules::tx::BroadcastMode::Sync.into(),
            })
            .await?;

        let commit = commit.into_inner().tx_response.unwrap();
        Ok(commit)
    }
}

fn has_insufficient_fee(raw_log: &str) -> bool {
    raw_log.contains("insufficient fees")
}

// from logs: "insufficient fees; got: 14867ujuno
// required: 17771ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9,444255ujuno: insufficient fee"
fn parse_suggested_fee(raw_log: &str) -> Option<u128> {
    // Step 1: Split the log message into "got" and "required" parts.
    let parts: Vec<&str> = raw_log.split("required: ").collect();

    // Make sure the log message is in the expected format.
    if parts.len() != 2 {
        return None;
    }

    // Step 2: Split the "got" part to extract the paid fee and denomination.
    let got_parts: Vec<&str> = parts[0].split_whitespace().collect();

    // Extract the paid fee and denomination.
    let paid_fee_with_denom = got_parts.last()?;
    let (_, denomination) =
        paid_fee_with_denom.split_at(paid_fee_with_denom.find(|c: char| !c.is_numeric())?);

    eprintln!("denom: {}", denomination);

    // Step 3: Iterate over each fee in the "required" part.
    let required_fees: Vec<&str> = parts[1].split(denomination).collect();

    eprintln!("required fees: {:?}", required_fees);

    // read until the first non-numeric character backwards on the first string
    let (_, suggested_fee) =
        required_fees[0].split_at(required_fees[0].rfind(|c: char| !c.is_numeric())?);
    eprintln!("suggested fee: {}", suggested_fee);

    // remove the first character if parsing errors, which can be a comma
    suggested_fee
        .parse::<u128>()
        .ok()
        .or(suggested_fee[1..].parse::<u128>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_suggested_fee() {
        let log = "insufficient fees; got: 14867ujuno required: 17771ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9,444255ujuno: insufficient fee";
        let fee = parse_suggested_fee(log).unwrap();
        assert_eq!(fee, 444255);
    }
}
