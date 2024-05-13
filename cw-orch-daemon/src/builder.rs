use crate::{
    log::print_if_log_disabled, senders::base_sender::SenderBuilder,
    senders::base_sender::SenderOptions, DaemonAsync, DaemonBuilder,
};
use std::sync::Arc;

use bitcoin::secp256k1::All;

use super::{error::DaemonError, senders::base_sender::Sender, state::DaemonState};
use cw_orch_core::environment::ChainInfoOwned;

/// The default deployment id if none is provided
pub const DEFAULT_DEPLOYMENT: &str = "default";

#[derive(Clone, Default)]
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
pub struct DaemonAsyncBuilder {
    // # Required
    pub(crate) chain: Option<ChainInfoOwned>,
    // # Optional
    pub(crate) deployment_id: Option<String>,

    /* Sender related options */
    /// Wallet sender
    /// Will be used in priority when set
    pub(crate) sender: Option<SenderBuilder<All>>,
    /// Specify Daemon Sender Options
    pub(crate) sender_options: SenderOptions,
}

impl DaemonAsyncBuilder {
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

    /// Set the mnemonic to use with this chain.
    /// Defaults to env variable depending on the environment.  
    ///
    /// Variables: LOCAL_MNEMONIC, TEST_MNEMONIC and MAIN_MNEMONIC
    pub fn mnemonic(&mut self, mnemonic: impl ToString) -> &mut Self {
        self.sender = Some(SenderBuilder::Mnemonic(mnemonic.to_string()));
        self
    }

    /// Specifies a sender to use with this chain
    /// This will be used in priority when set on the builder
    pub fn sender(&mut self, wallet: Sender<All>) -> &mut Self {
        self.sender = Some(SenderBuilder::Sender(wallet));
        self
    }

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

    /// Build a daemon
    pub async fn build(&self) -> Result<DaemonAsync, DaemonError> {
        let chain = self
            .chain
            .clone()
            .ok_or(DaemonError::BuilderMissing("chain information".into()))?;
        let deployment_id = self
            .deployment_id
            .clone()
            .unwrap_or(DEFAULT_DEPLOYMENT.to_string());
        let state = Arc::new(DaemonState::new(chain, deployment_id, false).await?);
        // if mnemonic provided, use it. Else use env variables to retrieve mnemonic
        let sender_options = self.sender_options.clone();

        let sender = match self.sender.clone() {
            Some(sender) => match sender {
                SenderBuilder::Mnemonic(mnemonic) => {
                    Sender::from_mnemonic_with_options(&state, &mnemonic, sender_options)?
                }
                SenderBuilder::Sender(mut sender) => {
                    sender.set_options(self.sender_options.clone());
                    sender
                }
            },
            None => Sender::new_with_options(&state, sender_options)?,
        };
        let daemon = DaemonAsync {
            state,
            sender: Arc::new(sender),
            querier: (),
        };
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
        }
    }
}
