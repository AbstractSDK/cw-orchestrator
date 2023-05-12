use std::rc::Rc;

use ibc_chain_registry::chain::ChainData;

use crate::prelude::{Daemon, DaemonBuilder};

use super::{
    super::{error::DaemonError, sender::Sender, state::DaemonState},
    core::SyncDaemon,
};

pub const DEFAULT_DEPLOYMENT: &str = "default";

#[derive(Clone, Default)]
/// Create [`SyncDaemon`] through [`SyncDaemonBuilder`]
/// ## Example
/// ```no_run
///     use cw_orch::prelude::{SyncDaemonBuilder, networks};
///
///     let SyncDaemon = SyncDaemonBuilder::default()
///         .chain(networks::LOCAL_JUNO)
///         .deployment_id("v0.1.0")
///         .build()
///         .unwrap();
/// ```
pub struct SyncDaemonBuilder {
    // # Required
    pub(crate) chain: Option<ChainData>,
    pub(crate) handle: Option<tokio::runtime::Handle>,
    // # Optional
    pub(crate) deployment_id: Option<String>,
    /// Wallet mnemonic
    pub(crate) mnemonic: Option<String>,
}

impl SyncDaemonBuilder {
    /// Set the chain the SyncDaemon will connect to
    pub fn chain(&mut self, chain: impl Into<ChainData>) -> &mut Self {
        self.chain = Some(chain.into());
        self
    }

    /// Set the deployment id to use for the SyncDaemon interactions
    /// Defaults to `default`
    pub fn deployment_id(&mut self, deployment_id: impl Into<String>) -> &mut Self {
        self.deployment_id = Some(deployment_id.into());
        self
    }

    /// Set the tokio runtime handle to use for the SyncDaemon
    ///
    /// ## Example
    /// ```no_run
    /// use cw_orch::prelude::SyncDaemon;
    /// use tokio::runtime::Runtime;
    /// let rt = Runtime::new().unwrap();
    /// let SyncDaemon = SyncDaemon::builder()
    ///     .handle(rt.handle())
    ///     // ...
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn handle(&mut self, handle: &tokio::runtime::Handle) -> &mut Self {
        self.handle = Some(handle.clone());
        self
    }

    /// Set the mnemonic to use with this chain.
    pub fn mnemonic(&mut self, mnemonic: impl ToString) -> &mut Self {
        self.mnemonic = Some(mnemonic.to_string());
        self
    }

    /// Build a SyncDaemon
    pub fn build(&self) -> Result<SyncDaemon, DaemonError> {
        let rt_handle = self
            .handle
            .clone()
            .ok_or(DaemonError::BuilderMissing("runtime handle".into()))?;
        // build the underlying daemon
        let daemon = rt_handle.block_on(DaemonBuilder::from(self.clone()).build())?;

        Ok(SyncDaemon { rt_handle, daemon })
    }
}
