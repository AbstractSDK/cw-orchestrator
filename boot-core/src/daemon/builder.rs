use std::rc::Rc;

use ibc_chain_registry::chain::ChainData;

use crate::{Daemon, DaemonError};

use super::{sender::Sender, state::DaemonState};

pub const DEFAULT_DEPLOYMENT: &str = "default";

#[derive(Clone, Default)]
/// Create [`Daemon`] through [`DaemonBuilderBuilder`]
/// ## Example
/// ```ignore
///     use boot_core::{DaemonBuilder, networks};
///
///     let daemon = DaemonBuilder::default()
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
    /// Optional wallet mnemonic
    pub(crate) mnemonic: Option<String>,
}

impl DaemonBuilder {
    /// Set the chain the daemon will connect to
    pub fn chain(&mut self, chain: impl Into<ChainData>) -> &mut Self {
        self.chain = Some(chain.into());
        self
    }

    /// Set the deployment id to use for the daemon interactions
    /// Defaults to `default`
    pub fn deployment_id(&mut self, deployment_id: impl Into<String>) -> &mut Self {
        self.deployment_id = Some(deployment_id.into());
        self
    }

    /// Set the tokio runtime handle to use for the daemon
    /// Defaults to the current runtime
    ///
    /// ## Example
    /// ```no_run
    /// use boot_core::Daemon;
    /// use tokio::runtime::Runtime;
    /// let rt = Runtime::new().unwrap();
    /// let daemon = Daemon::builder()
    ///     .handle(rt.handle())
    ///     // ...
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn handle(&mut self, handle: &tokio::runtime::Handle) -> &mut Self {
        self.handle = Some(handle.clone());
        self
    }

    pub fn mnemonic(&mut self, mnemonic: impl ToString) -> &mut Self {
        self.mnemonic = Some(mnemonic.to_string());
        self
    }

    /// Build a daemon
    pub fn build(&self) -> Result<Daemon, DaemonError> {
        let chain = self
            .chain
            .clone()
            .ok_or(DaemonError::BuilderMissing("chain information".into()))?;
        let rt_handle = self.handle.clone().ok_or(DaemonError::BuilderMissing("runtime handle".into()))?;
        let deployment_id = self
            .deployment_id
            .clone()
            .unwrap_or(DEFAULT_DEPLOYMENT.to_string());
        let state = Rc::new(rt_handle.block_on(DaemonState::new(chain, deployment_id))?);
        // if mnemonic provided, use it. Else use env variables to retrieve mnemonic
        let sender = if let Some(mnemonic) = &self.mnemonic {
            Sender::from_mnemonic(&state, mnemonic)?
        } else {
            Sender::new(&state)?
        };
        let daemon = Daemon {
            rt_handle,
            state,
            sender: Rc::new(sender),
        };
        Ok(daemon)
    }
}
