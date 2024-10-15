use cosmwasm_std::IbcOrder;
use cw_orch_interchain_core::env::ChainId;
use cw_orch_starship::Starship;
use dialoguer::Input;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

use crate::{interchain_env::DaemonInterchain, InterchainDaemonError};

/// Used for allowing multiple interaction types with the Daemon interchain environment
pub trait ChannelCreator: Clone {
    /// Creates a channel for the interchain environement
    /// Returns the connection id used for creating the channel
    fn create_ibc_channel(
        &self,
        src_chain: ChainId,
        dst_chain: ChainId,
        src_port: &PortId,
        dst_port: &PortId,
        version: &str,
        order: Option<IbcOrder>,
    ) -> Result<String, InterchainDaemonError>;

    /// Returns an interchain environment from the channel creator object
    fn interchain_env(&self) -> DaemonInterchain<Self>;
}

/// This is a struct for use with actual RPCs where you want to create you channel manually
#[derive(Clone)]
pub struct ChannelCreationValidator;

impl ChannelCreator for ChannelCreationValidator {
    fn create_ibc_channel(
        &self,
        src_chain: ChainId,
        dst_chain: ChainId,
        src_port: &PortId,
        dst_port: &PortId,
        version: &str,
        order: Option<IbcOrder>,
    ) -> Result<String, InterchainDaemonError> {
        // In a production script, we want the channel creation to be handled externally
        // That means the user at this point should go outside of cw-orch, create their channel and resume their deployments

        let connection_id: String = Input::new().with_prompt(
            format!("Please create a channel now between {src_chain}: {src_port} and {dst_chain}: {dst_port} with version {version} and order {order:?}. When you are done, please indicate the connection-id you used")).interact_text()?;

        Ok(connection_id)
    }

    fn interchain_env(&self) -> DaemonInterchain<Self> {
        panic!("To create an RPC based interchain environement, use DaemonInterchain::new(). Use the Starship::interchain_env() method for interacting with Starship")
    }
}

impl ChannelCreator for Starship {
    fn create_ibc_channel(
        &self,
        src_chain: ChainId,
        dst_chain: ChainId,
        src_port: &PortId,
        dst_port: &PortId,
        version: &str,
        order: Option<IbcOrder>,
    ) -> Result<String, InterchainDaemonError> {
        // The connection_id is decided upon automatically by starship and returned by the client
        let connection_id = self.rt_handle.block_on(self.client().create_channel(
            src_chain,
            dst_chain,
            src_port.as_str(),
            dst_port.as_str(),
            version,
            order,
        ))?;
        log::info!("Channel was created in starship !");

        Ok(connection_id)
    }

    fn interchain_env(&self) -> DaemonInterchain<Self> {
        DaemonInterchain::from_daemons(self.daemons.values().cloned().collect(), self)
    }
}
