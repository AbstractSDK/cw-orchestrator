//! This module contains the trait definition for an interchain analysis environment

use cosmwasm_std::{ensure, Binary, IbcOrder};
use cw_orch_core::{
    contract::interface_traits::ContractInstance,
    environment::{CwEnv, IndexResponse, TxHandler},
};
use ibc_relayer_types::core::{
    ics04_channel::packet::Sequence,
    ics24_host::identifier::{ChannelId, PortId},
};

use crate::{
    ack_parser::{AckParser, IbcAckParser},
    channel::{IbcPort, InterchainChannel},
    types::{
        parse::SuccessIbcPacket, ChannelCreationResult, ChannelCreationTransactionsResult,
        FullIbcPacketAnalysis, IbcPacketOutcome, IbcTxAnalysis, InternalChannelCreationResult,
        SimpleIbcPacketAnalysis,
    },
    IbcQueryHandler, InterchainError,
};

/// This struct contains information about an IBC channel creation process. It contains the same struct type for each step of the channel creation
/// One way of using this object is to use it as
/// ```ignore
///     ChannelCreation<cw_orch::daemon::CosmTxResponse>
/// ```
/// In this example the struct contains all transaction results for channel creation
/// Those transactions are usually sent by a validator
/// [More info about channel creation here](https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md)

pub struct ChannelCreation<R> {
    /// First step, channel creation open-initialization (src_chain)
    pub init: R,
    /// Second step, channel creation open-try (dst_chain)
    pub r#try: R,
    /// Third step, channel creation connect-acknowledgement (src_chain)
    pub ack: R,
    /// Fourth step, channel creation connect-confirm (dst_chain)
    pub confirm: R,
}

impl<R> ChannelCreation<R> {
    /// Helper for creation a new channel result
    pub fn new(init: R, t: R, ack: R, confirm: R) -> Self {
        Self {
            init,
            r#try: t,
            ack,
            confirm,
        }
    }
}

/// Alias to indicate functions await a chain id
pub type ChainId<'a> = &'a str;

/// This trait allows to extend `cw_orch::prelude::CwEnv` with interchain capabilities
/// The center of those capabilities is the ability to follow the execution of outgoing IBC packets
/// This enables users to script chain execution even throughout IBC executions which are more asynchronous than usual transactions.
/// With the following simple syntax,
/// ```ignore
///     let ibc_execution = interchain.wait_ibc("juno", tx_response).await?;
/// ```
/// users are even able to await and analyze the execution of all packets submitted in a single transactions
/// without having to deal with validator addresses, packet sequences, channels_ids or whatever other IBC jargon.
/// Everything is analyzed automatically and transmitted back to them in a simple analysis structure.
/// Other capabilities are offered, including :
/// - Handling multiple chains in one location (to avoid passing them as arguments each time)
/// - Creating IBC channels between chains
/// - Logging packet execution steps for easier debugging (failed acks, timeouts...)
pub trait InterchainEnv<Chain: IbcQueryHandler>: Clone {
    /// Type returned by the internal channel creation function
    /// Examples
    /// Daemon : This is empty, because hermes doesn't return anything after channel creation
    /// Mock : The whole ChannelCreation<AppResponse> result is returned because the transactions and channel ids cannot be queried after
    type ChannelCreationResult;

    /// Error type for transactions in an environment.
    type Error: 'static + Into<InterchainError> + std::fmt::Debug + std::error::Error + Sync + Send;

    /// Returns a chain if it's registered in the environment
    /// Returns an error if the provided chain doesn't exist
    /// The chain_id doesn't have to be the actual chain_id of the chain. The way this id is handled is implementation specific
    fn chain(&self, chain_id: impl ToString) -> Result<Chain, Self::Error>;

    /// This triggers channel creation between 2 chains
    /// Returns a channel creation receipt as well as as the connection_id on the src_chain side
    /// This code is only for internal use and for most cases shouldn't be used outside of the [InterchainEnv<Chain>::create_channel] function
    fn _internal_create_channel(
        &self,
        src_chain: ChainId,
        dst_chain: ChainId,
        src_port: &PortId,
        dst_port: &PortId,
        version: &str,
        order: Option<IbcOrder>,
    ) -> Result<InternalChannelCreationResult<Self::ChannelCreationResult>, Self::Error>;

    /// Queries channel creation txs as well
    /// Fills the connection_id field of the destination chain in the ibc_channel object
    /// Returns the channel ids as well as the transaction responses involved in the channel creation
    fn get_channel_creation_txs(
        &self,
        src_chain: ChainId,
        ibc_channel: &mut InterchainChannel<<Chain as IbcQueryHandler>::Handler>,
        channel_creation_result: Self::ChannelCreationResult,
    ) -> Result<ChannelCreationTransactionsResult<Chain>, Self::Error>;

    /// Creates a channel and returns the 4 transactions hashes for channel creation
    /// This function should be used in code to make sure the channel creation (+ eventual packet relaying) is awaited before continuing
    /// This shouldn't need to be re-implemented.
    fn create_channel(
        &self,
        src_chain: ChainId,
        dst_chain: ChainId,
        src_port: &PortId,
        dst_port: &PortId,
        version: &str,
        order: Option<IbcOrder>,
    ) -> Result<ChannelCreationResult<Chain>, InterchainError> {
        // We create a channel internally
        let InternalChannelCreationResult {
            result,
            src_connection_id,
        } = self
            ._internal_create_channel(src_chain, dst_chain, src_port, dst_port, version, order)
            .map_err(Into::into)?;

        // We create the temporary InterchainChannel Object
        let mut ibc_channel = InterchainChannel::new(
            IbcPort {
                chain: self.chain(src_chain).map_err(Into::into)?.ibc_handler(),
                chain_id: src_chain.to_string(),
                connection_id: Some(src_connection_id),
                port: src_port.clone(),
                channel: None,
            },
            IbcPort {
                chain: self.chain(dst_chain).map_err(Into::into)?.ibc_handler(),
                chain_id: dst_chain.to_string(),
                connection_id: None,
                port: dst_port.clone(),
                channel: None,
            },
        );

        // We get the channel creation txs
        let ChannelCreationTransactionsResult {
            src_channel_id,
            dst_channel_id,
            channel_creation_txs,
        } = self
            .get_channel_creation_txs(src_chain, &mut ibc_channel, result)
            .map_err(Into::into)?;

        // We get the connection on the receiving chain
        let dst_connection_id = channel_creation_txs
            .r#try
            .event_attr_value("channel_open_try", "connection_id")
            .map_err(InterchainError::StdError)?
            .clone();

        // We follow all packets that were created in these transactions
        let packet_results = (
            self.wait_ibc(src_chain, channel_creation_txs.init)
                .map_err(Into::into)?,
            self.wait_ibc(dst_chain, channel_creation_txs.r#try)
                .map_err(Into::into)?,
            self.wait_ibc(src_chain, channel_creation_txs.ack)
                .map_err(Into::into)?,
            self.wait_ibc(dst_chain, channel_creation_txs.confirm)
                .map_err(Into::into)?,
        );

        // We return them
        let packet_results = ChannelCreation {
            init: packet_results.0,
            r#try: packet_results.1,
            ack: packet_results.2,
            confirm: packet_results.3,
        };

        let (src_port, dst_port) = ibc_channel.get_mut_ordered_ports_from(src_chain)?;

        dst_port.connection_id = Some(dst_connection_id);
        src_port.channel = Some(src_channel_id);
        dst_port.channel = Some(dst_channel_id);

        Ok(ChannelCreationResult {
            interchain_channel: ibc_channel,
            channel_creation_txs: packet_results,
        })
    }

    /// This function creates a channel and returns the 4 transactions hashes for channel creation
    fn create_contract_channel(
        &self,
        src_contract: &dyn ContractInstance<Chain>,
        dst_contract: &dyn ContractInstance<Chain>,
        version: &str,
        order: Option<IbcOrder>,
    ) -> Result<ChannelCreationResult<Chain>, InterchainError> {
        let src_chain = src_contract.get_chain().chain_id();
        let dst_chain = dst_contract.get_chain().chain_id();

        let src_port = contract_port(src_contract);
        let dst_port = contract_port(dst_contract);

        let channel_creation =
            self.create_channel(&src_chain, &dst_chain, &src_port, &dst_port, version, order)?;

        Ok(channel_creation)
    }

    /// Follows every IBC packets sent out during a transaction
    /// This returns a packet analysis.
    ///
    /// For easier handling of the Interchain response, please use [`Self::check_ibc`]
    ///
    /// For more control over the ack parsing, please use [`Self::parse_ibc`]
    fn wait_ibc(
        &self,
        chain_id: ChainId,
        tx_response: <Chain as TxHandler>::Response,
    ) -> Result<IbcTxAnalysis<Chain>, Self::Error>;

    /// Follow every IBC packets sent out during the transaction
    /// Parses the acks according to usual ack formats (ICS20, Polytone, ICS-004)
    /// Errors if the acks and't be parsed, correspond to a failed result or there is a timeout
    /// If you only want to await without validation, use [`Self::wait_ibc`]
    fn check_ibc(
        &self,
        chain_id: ChainId,
        tx_response: <Chain as TxHandler>::Response,
    ) -> Result<IbcTxAnalysis<Chain>, InterchainError> {
        let tx_result = self.wait_ibc(chain_id, tx_response).map_err(Into::into)?;

        ensure!(
            !tx_result.get_success_packets()?.is_empty(),
            InterchainError::NoPacketsFound {}
        );

        tx_result.into_result()?;

        Ok(tx_result)
    }

    /// Follow every IBC packets sent out during the transaction
    /// Returns an object that is used to analyze packets according to different formats
    /// # Example
    ///
    /// ```no_run,ignore
    /// let mut result = interchain.parse_ibc("osmosis-1", tx_response)?;
    /// let first_polytone_ack = result.find_and_pop(&IbcAckParser::polytone_ack)?;
    /// // ... You can parse and pop other packets if any
    /// // This final call allows you to make sure you haven't forgotten to parse any packets
    /// result.stop()?;
    /// ```
    fn parse_ibc(
        &self,
        chain_id: ChainId,
        tx_response: <Chain as TxHandler>::Response,
    ) -> Result<AckParser<Chain>, InterchainError> {
        let tx_result = self.wait_ibc(chain_id, tx_response).map_err(Into::into)?;
        tx_result.analyze()
    }

    /// Follow the execution of a single IBC packet across the chain.
    /// It won't follow additional packets sent out during the transmission of this packet
    /// This is usually not used outside of the structure implementation, but is still available if needed
    fn follow_packet(
        &self,
        src_chain: ChainId,
        src_port: PortId,
        src_channel: ChannelId,
        dst_chain: ChainId,
        sequence: Sequence,
    ) -> Result<SimpleIbcPacketAnalysis<Chain>, Self::Error>;
}

/// format the port for a contract
pub fn contract_port<Chain: CwEnv>(contract: &dyn ContractInstance<Chain>) -> PortId {
    format!("wasm.{}", contract.addr_str().unwrap())
        .parse()
        .unwrap()
}

impl<Chain: CwEnv> IbcTxAnalysis<Chain> {
    /// Tries to parses all acknowledgements into polytone, ics20 and ics004 acks.
    /// Errors if some packet doesn't conform to those results.
    /// Use [`InterchainEnv::parse_ibc`] if you want to handle your own acks
    pub fn into_result(&self) -> Result<(), InterchainError> {
        self.packets.iter().try_for_each(|p| p.into_result())?;
        Ok(())
    }
}

impl<Chain: CwEnv> FullIbcPacketAnalysis<Chain> {
    /// Tries to parses all acknowledgements into polytone, ics20 and ics004 acks.
    /// Errors if some packet doesn't conform to those results.
    /// Use [`InterchainEnv::parse_ibc`] if you want to handle your own acks
    pub fn into_result(&self) -> Result<(), InterchainError> {
        match &self.outcome {
            IbcPacketOutcome::Success {
                ack,
                receive_tx,
                ack_tx,
            } => {
                receive_tx.into_result()?;
                ack_tx.into_result()?;

                if IbcAckParser::polytone_ack(ack).is_ok() {
                    return Ok(());
                }
                if IbcAckParser::ics20_ack(ack).is_ok() {
                    return Ok(());
                }
                if IbcAckParser::ics004_ack(ack).is_ok() {
                    return Ok(());
                }

                Err(InterchainError::AckDecodingFailed(
                    ack.clone(),
                    String::from_utf8_lossy(ack.as_slice()).to_string(),
                ))
            }
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }

    pub(crate) fn get_success_packets(
        &self,
    ) -> Result<Vec<SuccessIbcPacket<Chain>>, InterchainError> {
        match &self.outcome {
            IbcPacketOutcome::Success {
                ack,
                receive_tx,
                ack_tx,
            } => Ok([
                vec![SuccessIbcPacket {
                    send_tx: self.send_tx.clone().unwrap(),
                    packet_ack: ack.clone(),
                }],
                receive_tx.get_success_packets()?,
                ack_tx.get_success_packets()?,
            ]
            .concat()),
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }
}

pub(crate) fn decode_ack_error(ack: &Binary) -> InterchainError {
    InterchainError::AckDecodingFailed(
        ack.clone(),
        String::from_utf8_lossy(ack.as_slice()).to_string(),
    )
}
