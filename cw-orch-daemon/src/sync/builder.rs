use crate::RUNTIME;
use crate::{
    sender::{Sender, SenderBuilder, SenderOptions},
    DaemonAsyncBuilder,
};
use bitcoin::secp256k1::All;
use ibc_chain_registry::chain::{ChainData, FeeToken, Grpc};

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
    // # Optional
    pub(crate) handle: Option<tokio::runtime::Handle>,
    pub(crate) deployment_id: Option<String>,
    pub(crate) additional_grpc_url: Option<String>,
    pub(crate) gas_denom: Option<String>,
    pub(crate) gas_fee: Option<f64>,

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

    pub fn add_grpc_url(&mut self, url: &str) -> &mut Self {
        self.additional_grpc_url = Some(url.to_string());
        self
    }

    pub fn gas(&mut self, gas_denom: Option<&str>, gas_fee: Option<f64>) -> &mut Self {
        self.gas_denom = gas_denom.map(ToString::to_string);
        self.gas_fee = gas_fee.map(Into::into);
        self
    }

    /// Build a Daemon
    pub fn build(&self) -> Result<Daemon, DaemonError> {
        let rt_handle = self
            .handle
            .clone()
            .unwrap_or_else(|| RUNTIME.handle().clone());

        let mut chain = self.chain.clone().unwrap();

        // Override gas fee
        override_fee(&mut chain, self.gas_denom.clone(), self.gas_fee);
        // Override grpc_url
        override_grpc_url(&mut chain, self.additional_grpc_url.clone());

        // build the underlying daemon
        let daemon = rt_handle.block_on(DaemonAsyncBuilder::from(self.clone()).build())?;

        Ok(Daemon { rt_handle, daemon })
    }
}

fn override_fee(chain: &mut ChainData, denom: Option<String>, amount: Option<f64>) {
    let selected_fee = chain.fees.fee_tokens.first().cloned();
    let fee_denom = denom
        .clone()
        .or(selected_fee.clone().map(|s| s.denom))
        .unwrap();

    let mut fee = amount
        .map(|fee| FeeToken {
            denom: fee_denom.clone(),
            fixed_min_gas_price: fee,
            low_gas_price: fee,
            average_gas_price: fee,
            high_gas_price: fee,
        })
        .or(selected_fee)
        .unwrap();
    fee.denom = fee_denom;
    chain.fees.fee_tokens.insert(0, fee);
}

fn override_grpc_url(chain: &mut ChainData, grpc_url: Option<String>) {
    if let Some(grpc_url) = grpc_url {
        chain.apis.grpc.insert(
            0,
            Grpc {
                address: grpc_url,
                ..Default::default()
            },
        );
    }
}
