use super::DaemonQuerier;
use crate::{cosmos_modules, error::DaemonError};
use cosmos_modules::ibc_channel;
use cosmrs::proto::ibc::{
    applications::transfer::v1::{DenomTrace, QueryDenomTraceResponse},
    core::{
        channel::v1::QueryPacketCommitmentResponse,
        client::v1::{IdentifiedClientState, QueryClientStatesResponse},
        connection::v1::{IdentifiedConnection, State},
    },
    lightclients::tendermint::v1::ClientState,
};
use prost::Message;
use tonic::transport::Channel;

/// Querier for the Cosmos IBC module
pub struct Ibc {
    channel: Channel,
}

impl DaemonQuerier for Ibc {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Ibc {
    // ### Transfer queries ### //

    /// Get the trace of a specific denom
    pub async fn denom_trace(&self, hash: String) -> Result<DenomTrace, DaemonError> {
        let denom_trace: QueryDenomTraceResponse = cosmos_query!(
            self,
            ibc_transfer,
            denom_trace,
            QueryDenomTraceRequest { hash: hash }
        );
        Ok(denom_trace.denom_trace.unwrap())
    }

    // ### Client queries ###

    /// Get all the IBC clients for this daemon
    pub async fn clients(&self) -> Result<Vec<IdentifiedClientState>, DaemonError> {
        let ibc_clients: QueryClientStatesResponse = cosmos_query!(
            self,
            ibc_client,
            client_states,
            QueryClientStatesRequest { pagination: None }
        );
        Ok(ibc_clients.client_states)
    }

    /// Get the state of a specific IBC client
    pub async fn client_state(
        &self,
        client_id: impl ToString,
        // Add the necessary parameters here
    ) -> Result<cosmos_modules::ibc_client::QueryClientStateResponse, DaemonError> {
        let response: cosmos_modules::ibc_client::QueryClientStateResponse = cosmos_query!(
            self,
            ibc_client,
            client_state,
            QueryClientStateRequest {
                client_id: client_id.to_string(),
            }
        );
        Ok(response)
    }

    /// Get the consensus state of a specific IBC client
    pub async fn consensus_states(
        &self,
        client_id: impl ToString,
    ) -> Result<cosmos_modules::ibc_client::QueryConsensusStatesResponse, DaemonError> {
        let client_id = client_id.to_string();
        let response: cosmos_modules::ibc_client::QueryConsensusStatesResponse = cosmos_query!(
            self,
            ibc_client,
            consensus_states,
            QueryConsensusStatesRequest {
                client_id: client_id,
                pagination: None,
            }
        );
        Ok(response)
    }

    /// Get the consensus status of a specific IBC client
    pub async fn client_status(
        &self,
        client_id: impl ToString,
        // Add the necessary parameters here
    ) -> Result<cosmos_modules::ibc_client::QueryClientStatusResponse, DaemonError> {
        let response: cosmos_modules::ibc_client::QueryClientStatusResponse = cosmos_query!(
            self,
            ibc_client,
            client_status,
            QueryClientStatusRequest {
                client_id: client_id.to_string(),
            }
        );
        Ok(response)
    }

    /// Get the ibc client parameters
    pub async fn client_params(
        &self,
    ) -> Result<cosmos_modules::ibc_client::QueryClientParamsResponse, DaemonError> {
        let response: cosmos_modules::ibc_client::QueryClientParamsResponse =
            cosmos_query!(self, ibc_client, client_params, QueryClientParamsRequest {});
        Ok(response)
    }

    // ### Connection queries ###

    /// Query the IBC connections for a specific chain
    pub async fn connections(&self) -> Result<Vec<IdentifiedConnection>, DaemonError> {
        use cosmos_modules::ibc_connection::QueryConnectionsResponse;

        let ibc_connections: QueryConnectionsResponse = cosmos_query!(
            self,
            ibc_connection,
            connections,
            QueryConnectionsRequest { pagination: None }
        );
        Ok(ibc_connections.connections)
    }

    /// Search for open connections with a specific chain.
    pub async fn open_connections(
        &self,
        client_chain_id: impl ToString,
    ) -> Result<Vec<IdentifiedConnection>, DaemonError> {
        let connections = self.connections().await?;
        let mut open_connections = Vec::new();
        for connection in connections {
            if connection.state() == State::Open {
                open_connections.push(connection);
            }
        }

        // now search for the connections that use a client with the correct chain ids
        let mut filtered_connections = Vec::new();
        for connection in open_connections {
            let client_state = self.connection_client(&connection.id).await?;
            if client_state.chain_id == client_chain_id.to_string() {
                filtered_connections.push(connection);
            }
        }

        Ok(filtered_connections)
    }

    /// Get all the connections for this client
    pub async fn client_connections(
        &self,
        client_id: impl Into<String>,
    ) -> Result<Vec<String>, DaemonError> {
        use cosmos_modules::ibc_connection::QueryClientConnectionsResponse;

        let client_id = client_id.into();
        let ibc_client_connections: QueryClientConnectionsResponse = cosmos_query!(
            self,
            ibc_connection,
            client_connections,
            QueryClientConnectionsRequest {
                client_id: client_id.clone()
            }
        );

        Ok(ibc_client_connections.connection_paths)
    }

    /// Get the (tendermint) client state for a specific connection
    pub async fn connection_client(
        &self,
        connection_id: impl Into<String>,
    ) -> Result<ClientState, DaemonError> {
        use cosmos_modules::ibc_connection::QueryConnectionClientStateResponse;
        let connection_id = connection_id.into();

        let ibc_connection_client: QueryConnectionClientStateResponse = cosmos_query!(
            self,
            ibc_connection,
            connection_client_state,
            QueryConnectionClientStateRequest {
                connection_id: connection_id.clone()
            }
        );

        let client_state =
            ibc_connection_client
                .identified_client_state
                .ok_or(DaemonError::ibc_err(format!(
                    "error identifying client for connection {}",
                    connection_id
                )))?;

        let client_state = ClientState::decode(client_state.client_state.unwrap().value.as_slice())
            .map_err(|e| DaemonError::ibc_err(format!("error decoding client state: {}", e)))?;

        Ok(client_state)
    }

    // ### Channel queries ###

    /// Get the channel for a specific port and channel id
    pub async fn channel(
        &self,
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
    ) -> Result<ibc_channel::Channel, DaemonError> {
        use cosmos_modules::ibc_channel::QueryChannelResponse;

        let port_id = port_id.into();
        let channel_id = channel_id.into();
        let ibc_channel: QueryChannelResponse = cosmos_query!(
            self,
            ibc_channel,
            channel,
            QueryChannelRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
            }
        );

        ibc_channel.channel.ok_or(DaemonError::ibc_err(format!(
            "error fetching channel {} on port {}",
            channel_id, port_id
        )))
    }

    /// Get all the channels for a specific connection
    pub async fn connection_channels(
        &self,
        connection_id: impl Into<String>,
    ) -> Result<Vec<ibc_channel::IdentifiedChannel>, DaemonError> {
        use cosmos_modules::ibc_channel::QueryConnectionChannelsResponse;

        let connection_id = connection_id.into();
        let ibc_connection_channels: QueryConnectionChannelsResponse = cosmos_query!(
            self,
            ibc_channel,
            connection_channels,
            QueryConnectionChannelsRequest {
                connection: connection_id.clone(),
                pagination: None,
            }
        );

        Ok(ibc_connection_channels.channels)
    }

    /// Get the client state for a specific channel and port
    pub async fn channel_client_state(
        &self,
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
    ) -> Result<IdentifiedClientState, DaemonError> {
        use cosmos_modules::ibc_channel::QueryChannelClientStateResponse;

        let port_id = port_id.into();
        let channel_id = channel_id.into();
        let ibc_channel_client_state: QueryChannelClientStateResponse = cosmos_query!(
            self,
            ibc_channel,
            channel_client_state,
            QueryChannelClientStateRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
            }
        );

        ibc_channel_client_state
            .identified_client_state
            .ok_or(DaemonError::ibc_err(format!(
                "error identifying client for channel {} on port {}",
                channel_id, port_id
            )))
    }

    // ### Packet queries ###

    // Commitment

    /// Get all the packet commitments for a specific channel and port
    pub async fn packet_commitments(
        &self,
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
    ) -> Result<Vec<ibc_channel::PacketState>, DaemonError> {
        use cosmos_modules::ibc_channel::QueryPacketCommitmentsResponse;

        let port_id = port_id.into();
        let channel_id = channel_id.into();
        let ibc_packet_commitments: QueryPacketCommitmentsResponse = cosmos_query!(
            self,
            ibc_channel,
            packet_commitments,
            QueryPacketCommitmentsRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
                pagination: None,
            }
        );

        Ok(ibc_packet_commitments.commitments)
    }

    /// Get the packet commitment for a specific channel, port and sequence
    pub async fn packet_commitment(
        &self,
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
        sequence: u64,
    ) -> Result<QueryPacketCommitmentResponse, DaemonError> {
        let port_id = port_id.into();
        let channel_id = channel_id.into();
        let ibc_packet_commitment: QueryPacketCommitmentResponse = cosmos_query!(
            self,
            ibc_channel,
            packet_commitment,
            QueryPacketCommitmentRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
                sequence: sequence,
            }
        );

        Ok(ibc_packet_commitment)
    }

    // Receipt

    /// Returns if the packet is received on the connected chain.
    pub async fn packet_receipt(
        &self,
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
        sequence: u64,
    ) -> Result<bool, DaemonError> {
        let port_id = port_id.into();
        let channel_id = channel_id.into();
        let ibc_packet_receipt: ibc_channel::QueryPacketReceiptResponse = cosmos_query!(
            self,
            ibc_channel,
            packet_receipt,
            QueryPacketReceiptRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
                sequence: sequence,
            }
        );

        Ok(ibc_packet_receipt.received)
    }

    // Acknowledgement

    /// Get all the packet acknowledgements for a specific channel, port and commitment sequences
    pub async fn packet_acknowledgements(
        &self,
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
        packet_commitment_sequences: Vec<u64>,
    ) -> Result<Vec<ibc_channel::PacketState>, DaemonError> {
        use cosmos_modules::ibc_channel::QueryPacketAcknowledgementsResponse;

        let port_id = port_id.into();
        let channel_id = channel_id.into();
        let ibc_packet_acknowledgements: QueryPacketAcknowledgementsResponse = cosmos_query!(
            self,
            ibc_channel,
            packet_acknowledgements,
            QueryPacketAcknowledgementsRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
                packet_commitment_sequences: packet_commitment_sequences,
                pagination: None,
            }
        );

        Ok(ibc_packet_acknowledgements.acknowledgements)
    }

    /// Get the packet acknowledgement for a specific channel, port and sequence
    pub async fn packet_acknowledgement(
        &self,
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
        sequence: u64,
    ) -> Result<Vec<u8>, DaemonError> {
        let port_id = port_id.into();
        let channel_id = channel_id.into();
        let ibc_packet_acknowledgement: ibc_channel::QueryPacketAcknowledgementResponse = cosmos_query!(
            self,
            ibc_channel,
            packet_acknowledgement,
            QueryPacketAcknowledgementRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
                sequence: sequence,
            }
        );

        Ok(ibc_packet_acknowledgement.acknowledgement)
    }

    /// No acknowledgement exists on receiving chain for the given packet commitment sequence on sending chain.
    /// Returns the packet sequences that have not yet been received.
    pub async fn unreceived_packets(
        &self,
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
        packet_commitment_sequences: Vec<u64>,
    ) -> Result<Vec<u64>, DaemonError> {
        use cosmos_modules::ibc_channel::QueryUnreceivedPacketsResponse;

        let port_id = port_id.into();
        let channel_id = channel_id.into();
        let ibc_packet_unreceived: QueryUnreceivedPacketsResponse = cosmos_query!(
            self,
            ibc_channel,
            unreceived_packets,
            QueryUnreceivedPacketsRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
                packet_commitment_sequences: packet_commitment_sequences,
            }
        );

        Ok(ibc_packet_unreceived.sequences)
    }

    /// Returns the acknowledgement sequences that have not yet been received.
    /// Given a list of acknowledgement sequences from counterparty, determine if an ack on the counterparty chain has been received on the executing chain.
    /// Returns the list of acknowledgement sequences that have not yet been received.
    pub async fn unreceived_acks(
        &self,
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
        packet_ack_sequences: Vec<u64>,
    ) -> Result<Vec<u64>, DaemonError> {
        let port_id = port_id.into();
        let channel_id = channel_id.into();
        let ibc_packet_unreceived: ibc_channel::QueryUnreceivedAcksResponse = cosmos_query!(
            self,
            ibc_channel,
            unreceived_acks,
            QueryUnreceivedAcksRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
                packet_ack_sequences: packet_ack_sequences,
            }
        );

        Ok(ibc_packet_unreceived.sequences)
    }

    /// Returns the acknowledgement sequences that have not yet been received.
    /// Given a list of acknowledgement sequences from counterparty, determine if an ack on the counterparty chain has been received on the executing chain.
    /// Returns the list of acknowledgement sequences that have not yet been received.
    pub async fn next_sequence_receive(
        &self,
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
    ) -> Result<u64, DaemonError> {
        let port_id = port_id.into();
        let channel_id = channel_id.into();
        let next_receive: ibc_channel::QueryNextSequenceReceiveResponse = cosmos_query!(
            self,
            ibc_channel,
            next_sequence_receive,
            QueryNextSequenceReceiveRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
            }
        );

        Ok(next_receive.next_sequence_receive)
    }
}
