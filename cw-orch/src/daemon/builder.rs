use std::rc::Rc;

use ibc_chain_registry::chain::ChainData;

use crate::prelude::{DaemonAsync};

#[cfg(feature = "daemon")]
use crate::
    DaemonBuilder;


use super::{error::DaemonError, sender::Sender, state::DaemonState};

/// The default deployment id if none is provided
pub const DEFAULT_DEPLOYMENT: &str = "default";

#[derive(Clone, Default)]
/// Create [`DaemonAsync`] through [`DaemonAsyncBuilder`]
/// ## Example
/// ```no_run
/// # tokio_test::block_on(async {
/// use cw_orch::prelude::{DaemonAsyncBuilder, networks};
/// let daemon = DaemonAsyncBuilder::default()
///     .chain(networks::LOCAL_JUNO)
///     .deployment_id("v0.1.0")
///     .build()
///     .await.unwrap();
/// # })
/// ```
pub struct DaemonAsyncBuilder {
    // # Required
    pub(crate) chain: Option<ChainData>,
    // # Optional
    pub(crate) deployment_id: Option<String>,
    /// Wallet mnemonic
    pub(crate) mnemonic: Option<String>,
}

impl DaemonAsyncBuilder {
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

    /// Set the mnemonic to use with this chain.
    /// Defaults to env variable depending on the environment.
    ///
    /// Variables: LOCAL_MNEMONIC, TEST_MNEMONIC and MAIN_MNEMONIC
    pub fn mnemonic(&mut self, mnemonic: impl ToString) -> &mut Self {
        self.mnemonic = Some(mnemonic.to_string());
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
        let state = Rc::new(DaemonState::new(chain, deployment_id).await?);
        // if mnemonic provided, use it. Else use env variables to retrieve mnemonic
        let sender = if let Some(mnemonic) = &self.mnemonic {
            Sender::from_mnemonic(&state, mnemonic)?
        } else {
            Sender::new(&state)?
        };
        let daemon = DaemonAsync {
            state,
            sender: Rc::new(sender),
        };
        Ok(daemon)
    }
}

#[cfg(feature = "daemon")]
impl From<DaemonBuilder> for DaemonAsyncBuilder {
    fn from(value: DaemonBuilder) -> Self {
        DaemonAsyncBuilder {
            chain: value.chain,
            deployment_id: value.deployment_id,
            mnemonic: value.mnemonic,
        }
    }
}
