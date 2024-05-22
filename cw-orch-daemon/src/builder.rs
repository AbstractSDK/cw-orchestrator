use crate::{
    log::print_if_log_disabled,
    senders::{
        base_sender::{SenderBuilder, SenderOptions},
        sender_trait::SenderTrait,
    },
    DaemonAsync, DaemonAsyncBase, DaemonBuilder, Wallet,
};
use std::sync::Arc;

use bitcoin::secp256k1::All;

use super::{error::DaemonError, senders::base_sender::Sender, state::DaemonState};
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
pub struct DaemonAsyncBuilderBase<SenderGen: SenderTrait> {
    // # Required
    pub(crate) chain: Option<ChainInfoOwned>,
    // # Optional
    pub(crate) deployment_id: Option<String>,

    pub(crate) sender: Option<SenderGen>,

    // TODO, reallow rebuilding
    // /* Sender related options */
    // /// Wallet sender
    // /// Will be used in priority when set
    // pub(crate) sender: SenderGen::SenderBuilder,
    /// Specify Daemon Sender Options
    pub(crate) sender_options: SenderOptions,
}

pub type DaemonAsyncBuilder = DaemonAsyncBuilderBase<Wallet>;

impl<SenderGen: SenderTrait> Default for DaemonAsyncBuilderBase<SenderGen> {
    fn default() -> Self {
        Self {
            chain: Default::default(),
            deployment_id: Default::default(),
            sender_options: Default::default(),
            sender: None,
        }
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

    // TODO
    // /// Set the mnemonic to use with this chain.
    // /// Defaults to env variable depending on the environment.
    // ///
    // /// Variables: LOCAL_MNEMONIC, TEST_MNEMONIC and MAIN_MNEMONIC
    // pub fn mnemonic(&mut self, mnemonic: impl ToString) -> &mut Self {
    //     self.sender = Some(SenderBuilder::Mnemonic(mnemonic.to_string()));
    //     self
    // }

    // /// Specifies a sender to use with this chain
    // /// This will be used in priority when set on the builder
    // pub fn sender(&mut self, wallet: Sender<All>) -> &mut Self {
    //     self.sender = Some(SenderBuilder::Sender(wallet));
    //     self
    // }

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
    pub async fn build(&self) -> Result<DaemonAsyncBase<SenderGen>, DaemonError> {
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

        let sender = if let Some(sender) = self.sender.clone() {
            sender
        } else {
            SenderGen::build(self.sender_options.clone(), &state).map_err(Into::into)?
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
        }
    }
}
