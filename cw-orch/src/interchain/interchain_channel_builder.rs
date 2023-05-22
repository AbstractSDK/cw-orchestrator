use crate::daemon::error::DaemonError;
use crate::daemon::sync::core::Daemon;
use crate::interchain::hermes::Hermes;
use crate::interchain::infrastructure::NetworkId;
use crate::interchain::infrastructure::Port;
use crate::interchain::interchain_env::InterchainEnv;
use crate::interface_traits::ContractInstance;
use crate::state::ChainState;

use crate::daemon::queriers::DaemonQuerier;
use crate::daemon::queriers::Ibc;
use crate::interchain::docker::DockerHelper;
use crate::interchain::interchain_channel::InterchainChannel;
use crate::interchain::IcResult;

use tonic::transport::Channel;

use super::interchain_channel::IbcPort;

#[derive(Default)]
struct ChainChannelBuilder {
    pub chain_id: Option<NetworkId>,
    pub port: Option<Port>,
    pub grpc_channel: Option<Channel>,
}

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

    pub fn chain_a(&mut self, chain_id: impl Into<NetworkId>) -> &mut Self {
        self.chain_a.chain_id = Some(chain_id.into());
        self
    }

    pub fn chain_b(&mut self, chain_id: impl Into<NetworkId>) -> &mut Self {
        self.chain_b.chain_id = Some(chain_id.into());
        self
    }

    pub fn grpc_channel_a(&mut self, channel: Channel) -> &mut Self {
        self.chain_a.grpc_channel = Some(channel);
        self
    }

    pub fn grpc_channel_b(&mut self, channel: Channel) -> &mut Self {
        self.chain_b.grpc_channel = Some(channel);
        self
    }

    pub fn port_a(&mut self, port: impl Into<Port>) -> &mut Self {
        self.chain_a.port = Some(port.into());
        self
    }

    pub fn port_b(&mut self, port: impl Into<Port>) -> &mut Self {
        self.chain_b.port = Some(port.into());
        self
    }

    pub fn connection(&mut self, connection: impl Into<String>) -> &mut Self {
        self.connection_a = Some(connection.into());
        self
    }

    pub fn from_contracts(
        &mut self,
        contract_a: &dyn ContractInstance<Daemon>,
        contract_b: &dyn ContractInstance<Daemon>,
    ) -> &mut Self {
        self.chain_a(contract_a.get_chain().state().chain_id.clone());
        self.port_a(format!("wasm.{}", contract_a.address().unwrap()));
        self.grpc_channel_a(contract_a.get_chain().channel());

        self.chain_b(contract_b.get_chain().state().chain_id.clone());
        self.port_b(format!("wasm.{}", contract_b.address().unwrap()));
        self.grpc_channel_b(contract_b.get_chain().channel())
    }

    // The channel id id supposed to be the one created on the a side (you can interchange a and b at will to allow for that)
    // The connection between the 2 chains, The two chains_ids, and the port of chain a should be defined when building a channel object with this method
    // TODO enforce that with errors ?
    pub async fn channel_from(
        &self,
        channel_id_a: String,
    ) -> Result<InterchainChannel, DaemonError> {
        // First we need to construct the channels for chain a and chain b
        let grpc_channel_a = self.chain_a.grpc_channel.clone().unwrap();
        let grpc_channel_b = self.chain_b.grpc_channel.clone().unwrap();

        // Then we check that the channel indeed exists
        let registered_channel = Ibc::new(grpc_channel_a.clone())
            .channel(self.chain_a.port.clone().unwrap(), channel_id_a.clone())
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
                port: counterparty.port_id,
                channel: Some(counterparty.channel_id),
            },
        );

        Ok(channel)
    }

    // Create a channel on-chain and relay for it (using the Hermes relayer)
    // The connection, the two chain_ids and the two ports should be defined when creating this channel
    // TODO enforce that with errors ?
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
        let interchain_env = InterchainEnv::default()
            .add_custom_chain(self.chain_a.chain_id.clone().unwrap(), grpc_channel_a)?
            .add_custom_chain(self.chain_b.chain_id.clone().unwrap(), grpc_channel_b)?
            .clone();

        interchain_env
            .await_ibc_execution(
                self.chain_a.chain_id.clone().unwrap(),
                channel_creation_tx_a.txhash.clone(),
            )
            .await?;

        interchain_env
            .await_ibc_execution(
                self.chain_b.chain_id.clone().unwrap(),
                channel_creation_tx_b.txhash.clone(),
            )
            .await?;

        Ok(interchain)
    }
}
