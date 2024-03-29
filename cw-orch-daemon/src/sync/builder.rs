use crate::RUNTIME;
use crate::{
    sender::{Sender, SenderBuilder, SenderOptions},
    DaemonAsyncBuilder,
};
use bitcoin::secp256k1::All;
use ibc_chain_registry::chain::ChainData;

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

    /* Sender Options */
    /// Wallet sender
    pub(crate) sender: Option<SenderBuilder<All>>,
    /// Specify Daemon Sender Options
    pub(crate) sender_options: SenderOptions,
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

    /// Set a custom tokio runtime handle to use for the Daemon
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
        self.sender = Some(SenderBuilder::Mnemonic(mnemonic.to_string()));
        self
    }

    /// Specifies a sender to use with this chain
    /// This will be used in priority when set on the builder
    pub fn sender(&mut self, wallet: Sender<All>) -> &mut Self {
        self.sender = Some(SenderBuilder::Sender(wallet));
        self
    }

    /// Specifies wether authz should be used with this daemon
    pub fn authz_granter(&mut self, granter: impl ToString) -> &mut Self {
        self.sender_options.set_authz_granter(granter.to_string());
        self
    }

    /// Specifies wether feegrant should be used with this daemon
    pub fn fee_granter(&mut self, granter: impl ToString) -> &mut Self {
        self.sender_options.set_fee_granter(granter.to_string());
        self
    }

    /// Specifies the hd_index of the daemon sender
    pub fn hd_index(&mut self, index: u32) -> &mut Self {
        self.sender_options.hd_index = Some(index);
        self
    }

    /// Build a Daemon
    pub fn build(&self) -> Result<Daemon, DaemonError> {
        let rt_handle = self
            .handle
            .clone()
            .unwrap_or_else(|| RUNTIME.handle().clone());

        // build the underlying daemon
        let daemon = rt_handle.block_on(DaemonAsyncBuilder::from(self.clone()).build())?;

        Ok(Daemon { rt_handle, daemon })
    }
}
