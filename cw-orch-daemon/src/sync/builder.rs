use crate::senders::builder::SenderBuilder;

use crate::{DaemonAsyncBuilder, DaemonBase, DaemonState, Wallet, RUNTIME};
use cw_orch_core::environment::ChainInfoOwned;

use super::super::error::DaemonError;

#[derive(Clone)]
/// Create [`Daemon`] through [`DaemonBuilder`]
/// ## Example
/// ```no_run
///     use cw_orch_daemon::{networks, DaemonBuilder};
///
///     let Daemon = DaemonBuilder::new(networks::LOCAL_JUNO)
///         .deployment_id("v0.1.0")
///         .build()
///         .unwrap();
/// ```
pub struct DaemonBuilder {
    // # Required
    pub(crate) chain: ChainInfoOwned,

    // # Optional
    pub(crate) handle: Option<tokio::runtime::Handle>,
    pub(crate) deployment_id: Option<String>,
    pub(crate) state_path: Option<String>,
    /// State from rebuild or existing daemon
    pub(crate) state: Option<DaemonState>,
    pub(crate) write_on_change: Option<bool>,

    pub(crate) mnemonic: Option<String>,
}

impl DaemonBuilder {
    pub fn new(chain: impl Into<ChainInfoOwned>) -> Self {
        Self {
            chain: chain.into(),
            handle: None,
            deployment_id: None,
            state_path: None,
            state: None,
            write_on_change: None,
            mnemonic: None,
        }
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
    /// use cw_orch_daemon::{Daemon, networks};
    /// use tokio::runtime::Runtime;
    /// let rt = Runtime::new().unwrap();
    /// let Daemon = Daemon::builder(networks::LOCAL_JUNO)
    ///     .handle(rt.handle())
    ///     // ...
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn handle(&mut self, handle: &tokio::runtime::Handle) -> &mut Self {
        self.handle = Some(handle.clone());
        self
    }

    /// Overwrites the grpc_url used to interact with the chain
    pub fn grpc_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.chain.grpc_urls = vec![url.into()];
        self
    }

    /// Set the mnemonic used for the default Cosmos wallet
    pub fn mnemonic(&mut self, mnemonic: impl Into<String>) -> &mut Self {
        self.mnemonic = Some(mnemonic.into());
        self
    }

    /// Overwrites the gas denom used for broadcasting transactions.
    /// Behavior :
    /// - If no gas denom is provided, the first gas denom specified in the `self.chain` is used
    /// - If no gas fee is provided, the first gas fee specified in the self.chain is used
    pub fn gas(&mut self, gas_denom: Option<&str>, gas_price: Option<f64>) -> &mut Self {
        if let Some(denom) = gas_denom {
            self.chain.gas_denom = denom.to_string()
        }
        if let Some(price) = gas_price {
            self.chain.gas_price = price;
        }

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

    /// Specifies path to the daemon state file
    /// Defaults to env variable.
    ///
    /// Variable: STATE_FILE_ENV_NAME.
    #[allow(unused)]
    pub(crate) fn state_path(&mut self, path: impl ToString) -> &mut Self {
        self.state_path = Some(path.to_string());
        self
    }

    /// Build a Daemon with the default [`Wallet`] implementation.
    pub fn build(&self) -> Result<DaemonBase<Wallet>, DaemonError> {
        let rt_handle = self
            .handle
            .clone()
            .unwrap_or_else(|| RUNTIME.handle().clone());

        let builder = self.clone();

        // build the underlying daemon
        let daemon = rt_handle.block_on(DaemonAsyncBuilder::from(builder).build())?;

        Ok(DaemonBase { rt_handle, daemon })
    }

    /// Build a daemon
    pub fn build_sender<Sender: SenderBuilder>(
        &self,
        sender_options: Sender::Options,
    ) -> Result<DaemonBase<Sender>, DaemonError> {
        let rt_handle = self
            .handle
            .clone()
            .unwrap_or_else(|| RUNTIME.handle().clone());

        let builder = self.clone();

        // build the underlying daemon
        let daemon =
            rt_handle.block_on(DaemonAsyncBuilder::from(builder).build_sender(sender_options))?;

        Ok(DaemonBase { rt_handle, daemon })
    }
}

#[cfg(test)]
mod test {
    use cw_orch_core::environment::TxHandler;
    use cw_orch_networks::networks::OSMOSIS_1;

    use crate::{DaemonBase, DaemonBuilder, Wallet};
    pub const DUMMY_MNEMONIC:&str = "chapter wrist alcohol shine angry noise mercy simple rebel recycle vehicle wrap morning giraffe lazy outdoor noise blood ginger sort reunion boss crowd dutch";

    #[test]
    #[serial_test::serial]
    fn grpc_override() {
        let mut chain = OSMOSIS_1;
        chain.grpc_urls = &[];
        let daemon = DaemonBuilder::new(chain)
            .mnemonic(DUMMY_MNEMONIC)
            .grpc_url(OSMOSIS_1.grpc_urls[0])
            .build()
            .unwrap();

        assert_eq!(daemon.daemon.sender.chain_info.grpc_urls.len(), 1);
        assert_eq!(
            daemon.daemon.sender.chain_info.grpc_urls[0],
            OSMOSIS_1.grpc_urls[0].to_string(),
        );
    }

    #[test]
    #[serial_test::serial]
    fn fee_amount_override() {
        let fee_amount = 1.3238763;
        let daemon = DaemonBuilder::new(OSMOSIS_1)
            .mnemonic(DUMMY_MNEMONIC)
            .gas(None, Some(fee_amount))
            .build()
            .unwrap();
        println!("chain {:?}", daemon.daemon.sender.chain_info);

        assert_eq!(daemon.daemon.sender.chain_info.gas_price, fee_amount);
    }

    #[test]
    #[serial_test::serial]
    fn fee_denom_override() {
        let token = "my_token";
        let daemon = DaemonBuilder::new(OSMOSIS_1)
            .mnemonic(DUMMY_MNEMONIC)
            .gas(Some(token), None)
            .build()
            .unwrap();

        assert_eq!(daemon.daemon.sender.chain_info.gas_denom, token.to_string());
    }

    #[test]
    #[serial_test::serial]
    fn fee_override() {
        let fee_amount = 1.3238763;
        let token = "my_token";
        let daemon = DaemonBuilder::new(OSMOSIS_1)
            .mnemonic(DUMMY_MNEMONIC)
            .gas(Some(token), Some(fee_amount))
            .build()
            .unwrap();

        assert_eq!(daemon.daemon.sender.chain_info.gas_denom, token.to_string());

        assert_eq!(daemon.daemon.sender.chain_info.gas_price, fee_amount);
    }

    #[test]
    #[serial_test::serial]
    fn hd_index_re_generates_sender() -> anyhow::Result<()> {
        let daemon = DaemonBuilder::new(OSMOSIS_1)
            .mnemonic(DUMMY_MNEMONIC)
            .build()
            .unwrap();

        let indexed_daemon: DaemonBase<Wallet> = daemon
            .rebuild()
            .build_sender(daemon.wallet().options().hd_index(56))?;

        assert_ne!(
            daemon.sender().to_string(),
            indexed_daemon.sender().to_string()
        );

        Ok(())
    }
}
