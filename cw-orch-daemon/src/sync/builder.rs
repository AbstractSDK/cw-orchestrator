use ibc_chain_registry::chain::ChainData;

use crate::DaemonAsyncBuilder;

use super::{super::error::DaemonError, core::Daemon};

#[derive(Clone, Default)]
/// Create [`Daemon`] through [`DaemonBuilder`]
/// ## Example
/// ```no_run
///     use cw_orch_daemon::{networks, DaemonBuilder};
///
///     let Daemon = DaemonBuilder::default()
///         .chain(networks::LOCAL_JUNO)
///         .deployment_id("v0.1.0")
///         .build()
///         .unwrap();
/// ```
pub struct DaemonBuilder {
    // # Required
    pub(crate) chain: Option<ChainData>,
    pub(crate) handle: Option<tokio::runtime::Handle>,
    // # Optional
    pub(crate) deployment_id: Option<String>,
    /// Wallet mnemonic
    pub(crate) mnemonic: Option<String>,
}

impl DaemonBuilder {
    /// Set the chain the Daemon will connect to
    pub fn chain(&mut self, chain: impl Into<ChainData>) -> &mut Self {
        self.chain = Some(chain.into());
        self
    }

    /// Set the deployment id to use for the Daemon interactions
    /// Defaults to `default`
    pub fn deployment_id(&mut self, deployment_id: impl Into<String>) -> &mut Self {
        self.deployment_id = Some(deployment_id.into());
        self
    }

    /// Set the tokio runtime handle to use for the Daemon
    ///
    /// ## Example
    /// ```no_run
    /// use cw_orch_daemon::Daemon;
    /// use tokio::runtime::Runtime;
    /// let rt = Runtime::new().unwrap();
    /// let Daemon = Daemon::builder()
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

    /// Build a Daemon
    pub fn build(&self) -> Result<Daemon, DaemonError> {
        let rt_handle = self
            .handle
            .clone()
            .ok_or(DaemonError::BuilderMissing("runtime handle".into()))?;
        // build the underlying daemon

        let daemon = rt_handle.block_on(DaemonAsyncBuilder::from(self.clone()).build())?;

        Ok(Daemon { rt_handle, daemon })
    }
}
