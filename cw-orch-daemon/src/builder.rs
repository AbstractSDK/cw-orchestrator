use crate::{
    log::print_if_log_disabled, senders::sender_trait::SenderTrait, DaemonAsyncBase, DaemonBuilder,
    GrpcChannel, Wallet,
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
/// let daemon = DaemonAsyncBuilder::default()
///     .chain(networks::LOCAL_JUNO)
///     .deployment_id("v0.1.0")
///     .build()
///     .await.unwrap();
/// # })
/// ```
pub struct DaemonAsyncBuilderBase<SenderGen: SenderTrait = Wallet> {
    // # Required
    pub(crate) chain: Option<ChainInfoOwned>,
    // # Optional
    pub(crate) deployment_id: Option<String>,
    pub(crate) state_path: Option<String>,

    /* Rebuilder related options */
    pub(crate) state: Option<DaemonState>,

    // Sender options

    // # Optional indicated sender
    pub(crate) sender: Option<SenderGen>,

    /// Specify Daemon Sender Options
    pub(crate) sender_options: SenderGen::SenderOptions,
}

pub type DaemonAsyncBuilder = DaemonAsyncBuilderBase<Wallet>;

impl<SenderGen: SenderTrait> Default for DaemonAsyncBuilderBase<SenderGen> {
    fn default() -> Self {
        Self {
            chain: Default::default(),
            deployment_id: Default::default(),
            sender_options: Default::default(),
            sender: Default::default(),
            state_path: Default::default(),
            state: Default::default(),
        }
    }
}

impl DaemonAsyncBuilder {
    /// Specifies whether authz should be used with this daemon
    pub fn authz_granter(&mut self, granter: impl ToString) -> &mut Self {
        self.sender_options.set_authz_granter(granter);
        self
    }

    /// Specifies whether a fee grant should be used with this daemon
    pub fn fee_granter(&mut self, granter: impl ToString) -> &mut Self {
        self.sender_options.set_fee_granter(granter);
        self
    }

    /// Specifies the hd_index of the daemon sender
    pub fn hd_index(&mut self, index: u32) -> &mut Self {
        self.sender_options.hd_index = Some(index);
        self
    }

    /// Set the mnemonic to use with this chain.
    /// Defaults to env variable depending on the environment.
    ///
    /// Variables: LOCAL_MNEMONIC, TEST_MNEMONIC and MAIN_MNEMONIC
    pub fn mnemonic(&mut self, mnemonic: impl ToString) -> &mut Self {
        self.sender_options.mnemonic = Some(mnemonic.to_string());
        self
    }
}

impl<SenderGen: SenderTrait> DaemonAsyncBuilderBase<SenderGen> {
    /// Set the chain the daemon will connect to
    pub fn chain(&mut self, chain: impl Into<ChainInfoOwned>) -> &mut Self {
        self.chain = Some(chain.into());
        self
    }

    /// Set the deployment id to use for the daemon interactions
    /// Defaults to `default`
    pub fn deployment_id(&mut self, deployment_id: impl Into<String>) -> &mut Self {
        self.deployment_id = Some(deployment_id.into());
        self
    }

    /// Specifies a sender to use with this chain
    /// This will be used in priority when set on the builder
    pub fn sender<OtherSenderGen: SenderTrait>(
        &self,
        wallet: OtherSenderGen,
    ) -> DaemonAsyncBuilderBase<OtherSenderGen> {
        DaemonAsyncBuilderBase {
            chain: self.chain.clone(),
            deployment_id: self.deployment_id.clone(),
            state_path: self.state_path.clone(),
            state: self.state.clone(),
            sender: Some(wallet),
            sender_options: OtherSenderGen::SenderOptions::default(),
        }
    }

    /// Specifies path to the daemon state file
    /// Defaults to env variable.
    ///
    /// Variable: STATE_FILE_ENV_NAME.
    pub fn state_path(&mut self, path: impl ToString) -> &mut Self {
        self.state_path = Some(path.to_string());
        self
    }

    /// Build a daemon
    pub async fn build(&self) -> Result<DaemonAsyncBase<SenderGen>, DaemonError> {
        let chain_info = self
            .chain
            .clone()
            .ok_or(DaemonError::BuilderMissing("chain information".into()))?;
        let deployment_id = self
            .deployment_id
            .clone()
            .unwrap_or(DEFAULT_DEPLOYMENT.to_string());

        // find working grpc channel
        let grpc_channel =
            GrpcChannel::connect(&chain_info.grpc_urls, &chain_info.chain_id).await?;

        let state = match &self.state {
            Some(state) => {
                let mut state = state.clone();
                state.chain_data = chain_info.clone();
                state.deployment_id = deployment_id;
                state
            }
            None => {
                let json_file_path = self
                    .state_path
                    .clone()
                    .unwrap_or(DaemonState::state_file_path()?);

                DaemonState::new(json_file_path, chain_info.clone(), deployment_id, false)?
            }
        };
        // if mnemonic provided, use it. Else use env variables to retrieve mnemonic

        let sender = if let Some(sender) = &self.sender {
            let mut sender = sender.clone();
            sender.set_options(self.sender_options.clone());
            sender
        } else {
            SenderGen::build(chain_info, grpc_channel, self.sender_options.clone())
                .map_err(Into::into)?
        };

        let daemon = DaemonAsyncBase { state, sender };

        print_if_log_disabled()?;
        Ok(daemon)
    }
}

impl From<DaemonBuilder> for DaemonAsyncBuilder {
    fn from(value: DaemonBuilder) -> Self {
        DaemonAsyncBuilder {
            chain: value.chain,
            deployment_id: value.deployment_id,
            sender_options: value.sender_options,
            sender: value.sender,
            state: value.state,
            state_path: value.state_path,
        }
    }
}
