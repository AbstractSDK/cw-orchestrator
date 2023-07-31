//! Builder for the IntechainChannel object

use crate::daemon::Daemon;
use crate::daemon::DaemonError;
use crate::daemon::queriers::DaemonQuerier;
use crate::daemon::queriers::Ibc;
use crate::interface_traits::ContractInstance;
use crate::state::ChainState;

use crate::interchain_env::NetworkId;
use crate::packet_inspector::PacketInspector;

use crate::interchain_channel::InterchainChannel;
use crate::IcResult;

use ibc_relayer_types::core::ics24_host::identifier::ChannelId;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use tonic::transport::Channel;

use crate::interchain_channel::IbcPort;
use crate::interchain_env::contract_port;

#[derive(Default, Debug)]
struct ChainChannelBuilder {
    pub chain_id: Option<NetworkId>,
    pub port: Option<PortId>,
    pub grpc_channel: Option<Channel>,
}

/// Builder for a `InterchainChannel` Object
/// 2 actions can be executed with this builder :
/// 1. Create a tracking `InterchainChannel` object from an existing channel between two ibc-linked blockchains
/// 2.
///     a. Create a channel between 2 chains using a local Hermes instance (this is mostly used for testing) and THEN
///     b. Create a tracking `InterchainChannel` object for this specific channel
#[derive(Default)]
pub struct InterchainChannelBuilder {
    chain_a: ChainChannelBuilder,
    chain_b: ChainChannelBuilder,
    connection_a: Option<String>,
}

impl InterchainChannelBuilder {
    async fn get_hermes() -> IcResult<Hermes> {
        let docker_helper = DockerHelper::new().await?;
        docker_helper.get_hermes()
    }

    /// Sets the chain_id for the chain A
    pub fn chain_a(&mut self, chain_id: impl Into<NetworkId>) -> &mut Self {
        self.chain_a.chain_id = Some(chain_id.into());
        self
    }

    /// Sets the chain_id for the chain B
    pub fn chain_b(&mut self, chain_id: impl Into<NetworkId>) -> &mut Self {
        self.chain_b.chain_id = Some(chain_id.into());
        self
    }

    /// Sets the grpc channel_for the chain A
    pub fn grpc_channel_a(&mut self, channel: Channel) -> &mut Self {
        self.chain_a.grpc_channel = Some(channel);
        self
    }

    /// Sets the grpc channel_for the chain A
    pub fn grpc_channel_b(&mut self, channel: Channel) -> &mut Self {
        self.chain_b.grpc_channel = Some(channel);
        self
    }

    /// Sets the port that should be followed on chain a
    pub fn port_a(&mut self, port: PortId) -> &mut Self {
        self.chain_a.port = Some(port);
        self
    }

    /// Sets the port that should be followed on chain b
    pub fn port_b(&mut self, port: PortId) -> &mut Self {
        self.chain_b.port = Some(port);
        self
    }

    /// Sets the connection that should be monitored (this is the connection_id on chain A)
    pub fn connection(&mut self, connection: impl Into<String>) -> &mut Self {
        self.connection_a = Some(connection.into());
        self
    }

    /// Sets up the builder object from 2 contracts
    /// This simplifies the construction of the builder object when following a channel between 2 contracts
    pub fn from_contracts(
        &mut self,
        contract_a: &dyn ContractInstance<Daemon>,
        contract_b: &dyn ContractInstance<Daemon>,
    ) -> &mut Self {
        self.from_contract_a(contract_a);
        self.from_contract_b(contract_b)
    }

    /// Sets up the builder object from a contract on chainn A
    /// This simplifies the construction of the builder object when the port on chain A is associated with a contract
    pub fn from_contract_a(&mut self, contract_a: &dyn ContractInstance<Daemon>) -> &mut Self {
        self.chain_a(
            contract_a
                .get_chain()
                .state()
                .chain_data
                .chain_id
                .to_string(),
        );
        self.port_a(contract_port(contract_a));
        self.grpc_channel_a(contract_a.get_chain().channel())
    }

    /// Sets up the builder object from a contract on chain B
    /// This simplifies the construction of the builder object when the port on chain B is associated with a contract
    pub fn from_contract_b(&mut self, contract_b: &dyn ContractInstance<Daemon>) -> &mut Self {
        self.chain_b(
            contract_b
                .get_chain()
                .state()
                .chain_data
                .chain_id
                .to_string(),
        );
        self.port_b(contract_port(contract_b));
        self.grpc_channel_b(contract_b.get_chain().channel())
    }

    /// Creates an InterchainChannel object from an existing channel between 2 ports.
    /// This function requires the following struct members to be defined. Otherwise, it will panic
    /// - chain_id_a
    /// - chain_id_b
    /// - grpc_channel_a
    /// - grpc_channel_b
    /// - port_id_a
    /// A and B are symmetric in the InterchainChannel object
    /// So don't hesitate to interchange a and b when constructing the object if you only know the port and channel on one chain
    pub async fn channel_from(
        &self,
        channel_id_a: ChannelId,
    ) -> Result<InterchainChannel, DaemonError> {
        // First we need to construct the channels for chain a and chain b
        let grpc_channel_a = self.chain_a.grpc_channel.clone().unwrap();
        let grpc_channel_b = self.chain_b.grpc_channel.clone().unwrap();

        // Then we check that the channel indeed exists
        let registered_channel = Ibc::new(grpc_channel_a.clone())
            .channel(
                self.chain_a.port.clone().unwrap().to_string(),
                channel_id_a.to_string(),
            )
            .await?;
        let counterparty = registered_channel.counterparty.unwrap();

        let channel = InterchainChannel::new(
            registered_channel.connection_hops[0].clone(), // We suppose there is only one connection for this channel
            IbcPort {
                chain: grpc_channel_a,
                chain_id: self.chain_a.chain_id.clone().unwrap(),
                port: self.chain_a.port.clone().unwrap(),
                channel: Some(channel_id_a),
            },
            IbcPort {
                chain: grpc_channel_b,
                chain_id: self.chain_b.chain_id.clone().unwrap(),
                port: counterparty.port_id.parse().unwrap(),
                channel: Some(counterparty.channel_id.parse().unwrap()),
            },
        );

        Ok(channel)
    }

    /// Creates a channel AND creates an InterchainChannel object
    /// This function requires the following struct members to be defined. Otherwise, it will panic :
    /// - chain_id_a
    /// - chain_id_b
    /// - grpc_channel_a
    /// - grpc_channel_b
    /// - port_id_a
    /// - port_id_b
    /// You can optionnaly specify the connection between the 2 chains.
    /// If it's not provided, it will take the first connection the gRPC on chain A finds with a chain named `chain_id_b`
    /// Think function might block a long time because it waits until :
    /// 1. The channel is properly created
    /// 2. ALl IBC packets sent out during the channel creation procedure have been resolved (See `PacketInspector::await_ibc_execution` for more details)
    pub async fn create_channel(
        &self,
        channel_version: &str,
    ) -> Result<InterchainChannel, DaemonError> {
        let origin_chain_id = self.chain_a.chain_id.clone().unwrap();

        // We need to construct the channels for chain a and chain b
        let grpc_channel_a = self.chain_a.grpc_channel.clone().unwrap();
        let grpc_channel_b = self.chain_b.grpc_channel.clone().unwrap();

        // If the connection is not specified, we query it
        let connection = if let Some(connection) = &self.connection_a {
            connection.clone()
        } else {
            Ibc::new(grpc_channel_a.clone())
                .open_connections(self.chain_b.chain_id.clone().unwrap())
                .await?[0]
                .id
                .clone()
        };

        // Then we construct the InterchainChannel object
        let interchain = InterchainChannel::new(
            connection.clone(),
            IbcPort {
                chain: grpc_channel_a.clone(),
                chain_id: self.chain_a.chain_id.clone().unwrap(),
                port: self.chain_a.port.clone().unwrap(),
                channel: None,
            },
            IbcPort {
                chain: grpc_channel_b.clone(),
                chain_id: self.chain_b.chain_id.clone().unwrap(),
                port: self.chain_b.port.clone().unwrap(),
                channel: None,
            },
        );

        // First we get the last transactions for channel creation on the port, to make sure the tx we will intercept later is a new one
        let channel_creation_hashes = interchain
            .get_last_channel_creation_hash(origin_chain_id.clone())
            .await?;

        // Then we actually create a channel between the 2 ports
        Self::get_hermes()
            .await?
            .create_channel_raw(
                &connection,
                channel_version,
                &origin_chain_id,
                self.chain_a.port.clone().unwrap(),
                self.chain_b.port.clone().unwrap(),
            )
            .await;

        // Finally, we get the channel id from the chain creation events
        log::info!("Channel creation message sent to hermes, awaiting for channel creation end");

        // Then we make sure the channel is indeed created between the two chains
        let (channel_creation_tx_a, channel_creation_tx_b) = interchain
            .find_new_channel_creation_tx(origin_chain_id, &channel_creation_hashes)
            .await?;

        let src_channel_id = channel_creation_tx_a.get_events("channel_open_ack")[0]
            .get_first_attribute_value("channel_id")
            .unwrap();
        let dst_channel_id = channel_creation_tx_b.get_events("channel_open_confirm")[0]
            .get_first_attribute_value("channel_id")
            .unwrap();

        log::info!("Successfully created a channel between {} and {} on connection '{}' and channels {}:'{}'(txhash : {}) and {}:'{}' (txhash : {})", 
            self.chain_a.port.clone().unwrap(),
            self.chain_b.port.clone().unwrap(),
            connection,
            self.chain_a.chain_id.clone().unwrap(),
            src_channel_id,
            channel_creation_tx_a.txhash,
            self.chain_b.chain_id.clone().unwrap(),
            dst_channel_id,
            channel_creation_tx_b.txhash,
        );

        // We create and interchain analysis environment and register our daemons in it
        let packet_inspector = PacketInspector::from_channels(&vec![
            (self.chain_a.chain_id.clone().unwrap(), grpc_channel_a),
            (self.chain_b.chain_id.clone().unwrap(), grpc_channel_b),
        ])?;

        packet_inspector
            .await_ibc_execution(
                self.chain_a.chain_id.clone().unwrap(),
                channel_creation_tx_a.txhash.clone(),
            )
            .await?;

        packet_inspector
            .await_ibc_execution(
                self.chain_b.chain_id.clone().unwrap(),
                channel_creation_tx_b.txhash.clone(),
            )
            .await?;

        Ok(interchain)
    }
}
