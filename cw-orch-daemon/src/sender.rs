use crate::{
    env::DaemonEnvVars,
    proto::injective::ETHEREUM_COIN_TYPE,
    queriers::Bank,
    tx_broadcaster::{
        account_sequence_strategy, assert_broadcast_code_cosm_response, insufficient_fee_strategy,
        TxBroadcaster,
    },
};

use super::{
    cosmos_modules::{self, auth::BaseAccount},
    error::DaemonError,
    queriers::Node,
    tx_builder::TxBuilder,
    tx_resp::CosmTxResponse,
};
use crate::proto::injective::InjectiveEthAccount;

#[cfg(feature = "eth")]
use crate::proto::injective::InjectiveSigner;

use crate::{core::parse_cw_coins, keys::private::PrivateKey};
use cosmrs::{
    bank::MsgSend,
    crypto::secp256k1::SigningKey,
    proto::{cosmos::authz::v1beta1::MsgExec, traits::Message},
    tendermint::chain::Id,
    tx::{self, ModeInfo, Msg, Raw, SignDoc, SignMode, SignerInfo},
    AccountId, Any,
};
use cosmwasm_std::{coin, Addr, Coin};
use cw_orch_core::{
    environment::{ChainInfoOwned, ChainKind},
    log::local_target,
    CoreEnvVars, CwEnvError,
};

use crate::env::{LOCAL_MNEMONIC_ENV_NAME, MAIN_MNEMONIC_ENV_NAME, TEST_MNEMONIC_ENV_NAME};
use bitcoin::secp256k1::{All, Context, Secp256k1, Signing};
use std::{str::FromStr, sync::Arc};

use cosmos_modules::vesting::PeriodicVestingAccount;
use tonic::transport::Channel;

const GAS_BUFFER: f64 = 1.3;
const BUFFER_THRESHOLD: u64 = 200_000;
const SMALL_GAS_BUFFER: f64 = 1.4;

/// This enum allows for choosing which sender type will be constructed in a DaemonBuilder
#[derive(Clone)]
pub enum SenderBuilder<C: Signing + Context> {
    Sender(Sender<C>),
    Mnemonic(String),
}

/// A wallet is a sender of transactions, can be safely cloned and shared within the same thread.
pub type Wallet = Arc<Sender<All>>;

/// Signer of the transactions and helper for address derivation
/// This is the main interface for simulating and signing transactions
#[derive(Clone)]
pub struct Sender<C: Signing + Context> {
    pub private_key: PrivateKey,
    pub secp: Secp256k1<C>,
    /// gRPC channel
    pub grpc_channel: Channel,
    /// Information about the chain
    pub chain_info: ChainInfoOwned,
    pub(crate) options: SenderOptions,
}

/// Options for how txs should be constructed for this sender.
#[derive(Default, Clone)]
#[non_exhaustive]
pub struct SenderOptions {
    pub authz_granter: Option<String>,
    pub fee_granter: Option<String>,
    pub hd_index: Option<u32>,
}

impl SenderOptions {
    pub fn authz_granter(mut self, granter: impl ToString) -> Self {
        self.authz_granter = Some(granter.to_string());
        self
    }
    pub fn fee_granter(mut self, granter: impl ToString) -> Self {
        self.fee_granter = Some(granter.to_string());
        self
    }
    pub fn hd_index(mut self, index: u32) -> Self {
        self.hd_index = Some(index);
        self
    }
    pub fn set_authz_granter(&mut self, granter: impl ToString) {
        self.authz_granter = Some(granter.to_string());
    }
    pub fn set_fee_granter(&mut self, granter: impl ToString) {
        self.fee_granter = Some(granter.to_string());
    }
    pub fn set_hd_index(&mut self, index: u32) {
        self.hd_index = Some(index);
    }
}

impl Sender<All> {
    pub fn new(chain_info: ChainInfoOwned, channel: Channel) -> Result<Sender<All>, DaemonError> {
        Self::new_with_options(chain_info, channel, SenderOptions::default())
    }

    pub fn channel(&self) -> Channel {
        self.grpc_channel.clone()
    }

    pub fn new_with_options(
        chain_info: ChainInfoOwned,
        channel: Channel,
        options: SenderOptions,
    ) -> Result<Sender<All>, DaemonError> {
        let mnemonic = get_mnemonic_env(&chain_info.kind)?;

        Self::from_mnemonic_with_options(chain_info, channel, &mnemonic, options)
    }

    /// Construct a new Sender from a mnemonic with additional options
    pub fn from_mnemonic(
        chain_info: ChainInfoOwned,
        channel: Channel,
        mnemonic: &str,
    ) -> Result<Sender<All>, DaemonError> {
        Self::from_mnemonic_with_options(chain_info, channel, mnemonic, SenderOptions::default())
    }

    /// Construct a new Sender from a mnemonic with additional options
    pub fn from_mnemonic_with_options(
        chain_info: ChainInfoOwned,
        channel: Channel,
        mnemonic: &str,
        options: SenderOptions,
    ) -> Result<Sender<All>, DaemonError> {
        let secp = Secp256k1::new();
        let p_key: PrivateKey = PrivateKey::from_words(
            &secp,
            mnemonic,
            0,
            options.hd_index.unwrap_or(0),
            chain_info.network_info.coin_type,
        )?;

        let sender = Sender {
            chain_info,
            grpc_channel: channel,
            private_key: p_key,
            secp,
            options,
        };
        log::info!(
            target: &local_target(),
            "Interacting with {} using address: {}",
            sender.chain_info.chain_id,
            sender.pub_addr_str()?
        );
        Ok(sender)
    }

    pub fn set_authz_granter(&mut self, granter: impl Into<String>) {
        self.options.authz_granter = Some(granter.into());
    }

    pub fn set_fee_granter(&mut self, granter: impl Into<String>) {
        self.options.fee_granter = Some(granter.into());
    }

    pub fn set_options(&mut self, options: SenderOptions) {
        self.options = options;
    }

    fn cosmos_private_key(&self) -> SigningKey {
        SigningKey::from_slice(&self.private_key.raw_key()).unwrap()
    }

    pub fn pub_addr(&self) -> Result<AccountId, DaemonError> {
        Ok(AccountId::new(
            &self.chain_info.network_info.pub_address_prefix,
            &self.private_key.public_key(&self.secp).raw_address.unwrap(),
        )?)
    }

    pub fn address(&self) -> Result<Addr, DaemonError> {
        Ok(Addr::unchecked(self.pub_addr_str()?))
    }

    pub fn pub_addr_str(&self) -> Result<String, DaemonError> {
        Ok(self.pub_addr()?.to_string())
    }

    /// Returns the actual sender of every message sent.
    /// If an authz granter is set, returns the authz granter
    /// Else, returns the address associated with the current private key
    pub fn msg_sender(&self) -> Result<AccountId, DaemonError> {
        if let Some(sender) = &self.options.authz_granter {
            Ok(sender.parse()?)
        } else {
            self.pub_addr()
        }
    }

    pub async fn bank_send(
        &self,
        recipient: &str,
        coins: Vec<cosmwasm_std::Coin>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let msg_send = MsgSend {
            from_address: self.msg_sender()?,
            to_address: AccountId::from_str(recipient)?,
            amount: parse_cw_coins(&coins)?,
        };

        self.commit_tx(vec![msg_send], Some("sending tokens")).await
    }

    pub(crate) fn get_fee_token(&self) -> String {
        self.chain_info.gas_denom.to_string()
    }

    /// Compute the gas fee from the expected gas in the transaction
    /// Applies a Gas Buffer for including signature verification
    pub(crate) fn get_fee_from_gas(&self, gas: u64) -> Result<(u64, u128), DaemonError> {
        let mut gas_expected = if let Some(gas_buffer) = DaemonEnvVars::gas_buffer() {
            gas as f64 * gas_buffer
        } else if gas < BUFFER_THRESHOLD {
            gas as f64 * SMALL_GAS_BUFFER
        } else {
            gas as f64 * GAS_BUFFER
        };

        if let Some(min_gas) = DaemonEnvVars::min_gas() {
            gas_expected = (min_gas as f64).max(gas_expected);
        }
        let fee_amount = gas_expected * (self.chain_info.gas_price + 0.00001);

        Ok((gas_expected as u64, fee_amount as u128))
    }

    /// Computes the gas needed for submitting a transaction
    pub async fn calculate_gas(
        &self,
        tx_body: &tx::Body,
        sequence: u64,
        account_number: u64,
    ) -> Result<u64, DaemonError> {
        let fee = TxBuilder::build_fee(0u8, &self.chain_info.gas_denom, 0, self.options.clone())?;

        let auth_info = SignerInfo {
            public_key: self.private_key.get_signer_public_key(&self.secp),
            mode_info: ModeInfo::single(SignMode::Direct),
            sequence,
        }
        .auth_info(fee);

        let sign_doc = SignDoc::new(
            tx_body,
            &auth_info,
            &Id::try_from(self.chain_info.chain_id.to_string())?,
            account_number,
        )?;

        let tx_raw = self.sign(sign_doc)?;

        Node::new_async(self.channel())
            ._simulate_tx(tx_raw.to_bytes()?)
            .await
    }

    /// Simulates the transaction against an actual node
    /// Returns the gas needed as well as the fee needed for submitting a transaction
    pub async fn simulate(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> Result<(u64, Coin), DaemonError> {
        let timeout_height = Node::new_async(self.channel())._block_height().await? + 10u64;

        let tx_body = TxBuilder::build_body(msgs, memo, timeout_height);

        let tx_builder = TxBuilder::new(tx_body, self.chain_info.chain_id.clone());

        let gas_needed = tx_builder.simulate(self).await?;

        let (gas_for_submission, fee_amount) = self.get_fee_from_gas(gas_needed)?;
        let expected_fee = coin(fee_amount, self.get_fee_token());
        // During simulation, we also make sure the account has enough balance to submit the transaction
        // This is disabled by an env variable
        if DaemonEnvVars::wallet_balance_assertion() {
            self.assert_wallet_balance(&expected_fee).await?;
        }

        Ok((gas_for_submission, expected_fee))
    }

    pub async fn commit_tx<T: Msg>(
        &self,
        msgs: Vec<T>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let msgs = msgs
            .into_iter()
            .map(Msg::into_any)
            .collect::<Result<Vec<Any>, _>>()
            .unwrap();

        self.commit_tx_any(msgs, memo).await
    }

    pub async fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let timeout_height = Node::new_async(self.channel())._block_height().await? + 10u64;

        let msgs = if self.options.authz_granter.is_some() {
            // We wrap authz messages
            vec![Any {
                type_url: "/cosmos.authz.v1beta1.MsgExec".to_string(),
                value: MsgExec {
                    grantee: self.pub_addr_str()?,
                    msgs,
                }
                .encode_to_vec(),
            }]
        } else {
            msgs
        };

        let tx_body = TxBuilder::build_body(msgs, memo, timeout_height);

        let tx_builder = TxBuilder::new(tx_body, self.chain_info.chain_id.clone());

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

    pub async fn broadcast_tx(
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

    /// Allows for checking wether the sender is able to broadcast a transaction that necessitates the provided `gas`
    pub async fn has_enough_balance_for_gas(&self, gas: u64) -> Result<(), DaemonError> {
        let (_gas_expected, fee_amount) = self.get_fee_from_gas(gas)?;
        let fee_denom = self.get_fee_token();

        self.assert_wallet_balance(&coin(fee_amount, fee_denom))
            .await
    }

    /// Allows checking wether the sender has more funds than the provided `fee` argument
    #[async_recursion::async_recursion(?Send)]
    async fn assert_wallet_balance(&self, fee: &Coin) -> Result<(), DaemonError> {
        let chain_info = self.chain_info.clone();

        let bank = Bank::new_async(self.channel());
        let balance = bank
            ._balance(self.address()?, Some(fee.denom.clone()))
            .await?[0]
            .clone();

        log::debug!(
            "Checking balance {} on chain {}, address {}. Expecting {}{}",
            balance.amount,
            chain_info.chain_id,
            self.address()?,
            fee,
            fee.denom
        );
        let parsed_balance = coin(balance.amount.parse()?, balance.denom);

        if parsed_balance.amount >= fee.amount {
            log::debug!("The wallet has enough balance to deploy");
            return Ok(());
        }

        // If there is not enough asset balance, we need to warn the user
        println!(
            "Not enough funds on chain {} at address {} to deploy the contract. 
                Needed: {}{} but only have: {}.
                Press 'y' when the wallet balance has been increased to resume deployment",
            chain_info.chain_id,
            self.address()?,
            fee,
            fee.denom,
            parsed_balance
        );

        if CoreEnvVars::manual_interaction() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.to_lowercase().contains('y') {
                // We retry asserting the balance
                self.assert_wallet_balance(fee).await
            } else {
                Err(DaemonError::NotEnoughBalance {
                    expected: fee.clone(),
                    current: parsed_balance,
                })
            }
        } else {
            println!("No Manual Interactions, defaulting to 'no'");
            return Err(DaemonError::NotEnoughBalance {
                expected: fee.clone(),
                current: parsed_balance,
            });
        }
    }
}

fn get_mnemonic_env(chain_kind: &ChainKind) -> Result<String, CwEnvError> {
    match chain_kind {
        ChainKind::Local => DaemonEnvVars::local_mnemonic(),
        ChainKind::Testnet => DaemonEnvVars::test_mnemonic(),
        ChainKind::Mainnet => DaemonEnvVars::main_mnemonic(),
    }
    .ok_or(CwEnvError::EnvVarNotPresentNamed(
        get_mnemonic_env_name(chain_kind).to_string(),
    ))
}

fn get_mnemonic_env_name(chain_kind: &ChainKind) -> &str {
    match chain_kind {
        ChainKind::Local => LOCAL_MNEMONIC_ENV_NAME,
        ChainKind::Testnet => TEST_MNEMONIC_ENV_NAME,
        ChainKind::Mainnet => MAIN_MNEMONIC_ENV_NAME,
    }
}
