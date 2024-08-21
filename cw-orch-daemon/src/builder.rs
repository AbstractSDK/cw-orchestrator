use std::sync::Arc;

use crate::{
    log::print_if_log_disabled,
    network_config,
    senders::{builder::SenderBuilder, CosmosOptions, CosmosWalletKey},
    DaemonAsyncBase, DaemonBuilder, DaemonStateFile, Wallet,
};

use super::{error::DaemonError, state::DaemonState};
use cw_orch_core::environment::ChainInfoOwned;
/// The default deployment id if none is provided
pub const DEFAULT_DEPLOYMENT: &str = "default";

#[derive(Clone)]
/// Create [`DaemonAsync`] through [`DaemonAsyncBuilder`]
/// ## Example
/// ```no_run
/// # tokio_test::block_on(async {
/// use cw_orch_daemon::{DaemonAsyncBuilder, networks};
/// let daemon = DaemonAsyncBuilder::new(networks::LOCAL_JUNO)
///     .deployment_id("v0.1.0")
///     .build()
///     .await.unwrap();
/// # })
/// ```
pub struct DaemonAsyncBuilder {
    // # Required
    pub(crate) chain: ChainInfoOwned,
    // # Optional
    pub(crate) deployment_id: Option<String>,
    pub(crate) state_path: Option<String>,
    /// State from rebuild or existing daemon
    pub(crate) state: Option<DaemonState>,
    pub(crate) write_on_change: Option<bool>,
    pub(crate) is_test: bool,

    pub(crate) mnemonic: Option<String>,
}

impl DaemonAsyncBuilder {
    pub fn new(chain: impl Into<ChainInfoOwned>) -> Self {
        Self {
            chain: chain.into(),
            deployment_id: None,
            state_path: None,
            state: None,
            write_on_change: None,
            mnemonic: None,
            is_test: false,
        }
    }

    /// Set the deployment id to use for the daemon interactions
    /// Defaults to `default`
    pub fn deployment_id(&mut self, deployment_id: impl Into<String>) -> &mut Self {
        self.deployment_id = Some(deployment_id.into());
        self
    }

    /// Reuse already existent [`DaemonState`]
    /// Useful for multi-chain scenarios
    pub fn state(&mut self, state: DaemonState) -> &mut Self {
        self.state = Some(state);
        self
    }

    /// Whether to write on every change of the state
    /// If `true` - writes to a file on every change
    /// If `false` - writes to a file when all Daemons dropped this [`DaemonState`] or [`DaemonState::force_write`] used
    /// Defaults to `true`
    pub fn write_on_change(&mut self, write_on_change: bool) -> &mut Self {
        self.write_on_change = Some(write_on_change);
        self
    }

    /// Set the mnemonic used for the default Cosmos wallet
    pub fn mnemonic(&mut self, mnemonic: impl Into<String>) -> &mut Self {
        self.mnemonic = Some(mnemonic.into());
        self
    }

    /// Overwrite the chain info
    pub fn chain(&mut self, chain: impl Into<ChainInfoOwned>) -> &mut Self {
        self.chain = chain.into();
        self
    }

    /// Set daemon as testing daemon
    /// when set to `true` will use temporary file for state
    pub fn is_test(&mut self, is_test: bool) -> &mut Self {
        self.is_test = is_test;
        self
    }

    /// Specifies path to the daemon state file
    /// Defaults to env variable.
    ///
    /// Variable: STATE_FILE_ENV_NAME.
    #[allow(unused)]
    pub(crate) fn state_path(&mut self, path: impl ToString) -> &mut Self {
        self.state_path = Some(path.to_string());
        self
    }

    /// Build a daemon with provided mnemonic or env-var mnemonic
    pub async fn build(&self) -> Result<DaemonAsyncBase<Wallet>, DaemonError> {
        let chain_info = if let Some(network_config) = network_config::load(&self.chain.chain_id) {
            Arc::new(network_config.apply_to(self.chain.clone()))
        } else {
            Arc::new(self.chain.clone())
        };

        let state = self.build_state(chain_info.clone())?;
        // if mnemonic provided, use it. Else use env variables to retrieve mnemonic

        let options = CosmosOptions {
            key: self.mnemonic.as_ref().map_or(CosmosWalletKey::Env, |m| {
                CosmosWalletKey::Mnemonic(m.clone())
            }),
            ..Default::default()
        };
        let sender = options.build(&chain_info).await?;

        let daemon = DaemonAsyncBase::new(sender, state);

        print_if_log_disabled()?;
        Ok(daemon)
    }

    /// Build a daemon
    pub async fn build_sender<T: SenderBuilder>(
        &self,
        sender_options: T,
    ) -> Result<DaemonAsyncBase<T::Sender>, DaemonError> {
        let chain_info = if let Some(network_config) = network_config::load(&self.chain.chain_id) {
            Arc::new(network_config.apply_to(self.chain.clone()))
        } else {
            Arc::new(self.chain.clone())
        };

        let state = self.build_state(chain_info.clone())?;

        let sender = sender_options
            .build(&chain_info)
            .await
            .map_err(Into::into)?;

        let daemon = DaemonAsyncBase::new(sender, state);

        print_if_log_disabled()?;
        Ok(daemon)
    }

    /// Returns a built state
    fn build_state(&self, chain_info: Arc<ChainInfoOwned>) -> Result<DaemonState, DaemonError> {
        let deployment_id = self
            .deployment_id
            .clone()
            .unwrap_or(DEFAULT_DEPLOYMENT.to_string());

        let state = match &self.state {
            Some(state) => {
                let mut state = state.clone();
                state.chain_data = chain_info;
                state.deployment_id = deployment_id;
                if let Some(write_on_change) = self.write_on_change {
                    state.write_on_change = write_on_change;
                }
                // It's most likely a new chain, need to "prepare" json state for writes
                if let DaemonStateFile::FullAccess { json_file_state } = &state.json_state {
                    let mut json_file_lock = json_file_state.lock().unwrap();
                    json_file_lock.prepare(
                        &state.chain_data.chain_id,
                        &state.chain_data.network_info.chain_name,
                        &state.deployment_id,
                    );
                    if state.write_on_change {
                        json_file_lock.force_write();
                    }
                }
                state
            }
            None => {
                let json_file_path = match &self.state_path {
                    Some(path) => path.clone(),
                    None => {
                        if self.is_test {
                            crate::gen_temp_file_path()
                                .into_os_string()
                                .into_string()
                                .unwrap()
                        } else {
                            DaemonState::state_file_path()?
                        }
                    }
                };

                DaemonState::new(
                    json_file_path,
                    &chain_info,
                    deployment_id,
                    false,
                    self.write_on_change.unwrap_or(true),
                )?
            }
        };
        Ok(state)
    }
}

impl From<DaemonBuilder> for DaemonAsyncBuilder {
    fn from(value: DaemonBuilder) -> Self {
        DaemonAsyncBuilder {
            chain: value.chain,
            deployment_id: value.deployment_id,
            state: value.state,
            state_path: value.state_path,
            write_on_change: value.write_on_change,
            mnemonic: value.mnemonic,
            is_test: value.is_test,
        }
    }
}
