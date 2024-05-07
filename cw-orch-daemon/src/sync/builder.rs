use std::sync::{Arc, Mutex};

use crate::{
    sender::{Sender, SenderBuilder, SenderOptions},
    DaemonAsyncBuilder,
};
use crate::{DaemonState, RUNTIME};
use bitcoin::secp256k1::All;
use cw_orch_core::environment::ChainInfoOwned;

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
    pub(crate) chain: Option<ChainInfoOwned>,
    // # Optional
    pub(crate) handle: Option<tokio::runtime::Handle>,
    pub(crate) deployment_id: Option<String>,
    pub(crate) overwrite_grpc_url: Option<String>,
    pub(crate) gas_denom: Option<String>,
    pub(crate) gas_fee: Option<f64>,
    pub(crate) state_path: Option<String>,

    /* Sender Options */
    /// Wallet sender
    pub(crate) sender: Option<SenderBuilder<All>>,
    /// Specify Daemon Sender Options
    pub(crate) sender_options: SenderOptions,

    /* Rebuilder related options */
    pub(crate) state: Option<Arc<Mutex<DaemonState>>>,
}

impl DaemonBuilder {
    /// Set the chain the Daemon will connect to
    pub fn chain(&mut self, chain: impl Into<ChainInfoOwned>) -> &mut Self {
        self.chain = Some(chain.into());
        self
    }

    /// Set the deployment id to use for the Daemon interactions
    /// Defaults to `default`
    ///
    /// This field is ignored for rebuilt daemon and deployment id of the original daemon used instead
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

    /// Overwrites the grpc_url used to interact with the chain
    pub fn grpc_url(&mut self, url: &str) -> &mut Self {
        self.overwrite_grpc_url = Some(url.to_string());
        self
    }

    /// Overwrites the gas denom used for broadcasting transactions.
    /// Behavior :
    /// - If no gas denom is provided, the first gas denom specified in the `self.chain` is used
    /// - If no gas fee is provided, the first gas fee specified in the self.chain is used
    pub fn gas(&mut self, gas_denom: Option<&str>, gas_fee: Option<f64>) -> &mut Self {
        self.gas_denom = gas_denom.map(ToString::to_string);
        self.gas_fee = gas_fee.map(Into::into);
        self
    }

    /// Specifies path to the daemon state file
    /// Defaults to env variable.
    ///
    /// Variable: STATE_FILE_ENV_NAME.
    ///
    /// This field is ignored for rebuilt daemon and path of the original daemon used instead
    pub fn state_path(&mut self, path: impl ToString) -> &mut Self {
        self.state_path = Some(path.to_string());
        self
    }

    /// Build a Daemon
    pub fn build(&self) -> Result<Daemon, DaemonError> {
        let rt_handle = self
            .handle
            .clone()
            .unwrap_or_else(|| RUNTIME.handle().clone());

        let mut chain = self
            .chain
            .clone()
            .ok_or(DaemonError::BuilderMissing("chain information".into()))?;

        // Override gas fee
        overwrite_fee(&mut chain, self.gas_denom.clone(), self.gas_fee);
        // Override grpc_url
        overwrite_grpc_url(&mut chain, self.overwrite_grpc_url.clone());

        let mut builder = self.clone();
        builder.chain = Some(chain);

        // build the underlying daemon
        let daemon = rt_handle.block_on(DaemonAsyncBuilder::from(builder).build())?;

        Ok(Daemon { rt_handle, daemon })
    }
}

fn overwrite_fee(chain: &mut ChainInfoOwned, denom: Option<String>, amount: Option<f64>) {
    if let Some(denom) = denom {
        chain.gas_denom = denom.to_string()
    }
    chain.gas_price = amount.unwrap_or(chain.gas_price);
}

fn overwrite_grpc_url(chain: &mut ChainInfoOwned, grpc_url: Option<String>) {
    if let Some(grpc_url) = grpc_url {
        chain.grpc_urls = vec![grpc_url.to_string()]
    }
}

#[cfg(test)]
mod test {
    use cw_orch_networks::networks::OSMOSIS_1;

    use crate::DaemonBuilder;
    pub const DUMMY_MNEMONIC:&str = "chapter wrist alcohol shine angry noise mercy simple rebel recycle vehicle wrap morning giraffe lazy outdoor noise blood ginger sort reunion boss crowd dutch";

    #[test]
    #[serial_test::serial]
    fn grpc_override() {
        let mut chain = OSMOSIS_1;
        chain.grpc_urls = &[];
        let daemon = DaemonBuilder::default()
            .chain(chain)
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
        let daemon = DaemonBuilder::default()
            .chain(OSMOSIS_1)
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
        let daemon = DaemonBuilder::default()
            .chain(OSMOSIS_1)
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
        let daemon = DaemonBuilder::default()
            .chain(OSMOSIS_1)
            .mnemonic(DUMMY_MNEMONIC)
            .gas(Some(token), Some(fee_amount))
            .build()
            .unwrap();

        assert_eq!(daemon.daemon.sender.chain_info.gas_denom, token.to_string());

        assert_eq!(daemon.daemon.sender.chain_info.gas_price, fee_amount);
    }
}
