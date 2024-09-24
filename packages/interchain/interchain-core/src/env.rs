//! This module contains the trait definition for an interchain analysis environment

use cosmwasm_std::{Binary, IbcOrder};
use cw_orch_core::{
    contract::interface_traits::ContractInstance,
    environment::{CwEnv, Environment, IndexResponse, TxHandler},
};
use ibc_relayer_types::core::{
    ics04_channel::packet::Sequence,
    ics24_host::identifier::{ChannelId, PortId},
};

use crate::{
    ack_parser::IbcAckParser,
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
/// With the following simple syntax, users are even able to await and analyze the execution of all packets submitted in a single transactions
/// without having to deal with validator addresses, packet sequences, channels_ids or whatever other IBC jargon.
/// Everything is analyzed automatically and transmitted back to them in a simple analysis structure.
/// Other capabilities are offered, including :
/// - Handling multiple chains in one location (to avoid passing them as arguments each time)
/// - Creating IBC channels between chains
/// - Logging packet execution steps for easier debugging (failed acks, timeouts...)
///
/// ``` rust
/// use cosmwasm_std::{coin, CosmosMsg, IbcMsg, IbcTimeout, IbcTimeoutBlock};
/// use cw_orch::prelude::*;
/// use cw_orch::mock::cw_multi_test::Executor;
/// use cw_orch_interchain::prelude::*;
/// use ibc_relayer_types::core::ics24_host::identifier::PortId;

/// let interchain = MockInterchainEnv::new(vec![("juno-1", "sender"), ("stargaze-1", "sender")]);

/// let channel = interchain.create_channel(
///     "juno-1",
///     "stargaze-1",
///     &PortId::transfer(),
///     &PortId::transfer(),
///     "ics20-1",
///     None,
/// ).unwrap();
/// let juno = interchain.get_chain("juno-1").unwrap();
/// let stargaze = interchain.get_chain("stargaze-1").unwrap();

/// let channel = channel
///     .interchain_channel
///     .get_ordered_ports_from("juno-1").unwrap();

/// juno.add_balance(&juno.sender_addr(), vec![coin(100_000, "ujuno")]).unwrap();
/// let tx_resp = juno.app.borrow_mut().execute(
///     juno.sender_addr(),
///     CosmosMsg::Ibc(IbcMsg::Transfer {
///         channel_id: channel.0.channel.unwrap().to_string(),
///         to_address: stargaze.sender_addr().to_string(),
///         amount: coin(100_000, "ujuno"),
///         timeout: IbcTimeout::with_block(IbcTimeoutBlock {
///             revision: 1,
///             height: stargaze.block_info().unwrap().height + 1,
///         }),
///         memo: None,
///     }),
/// ).unwrap();

/// // This makes sure that the packets arrive successfully and present a success ack
/// let result = interchain.await_and_check_packets("juno-1", tx_resp).unwrap();
/// ```    
pub trait InterchainEnv<Chain: IbcQueryHandler>: Clone {
    /// Type returned by the internal channel creation function
    /// Examples
    /// Daemon : This is empty, because hermes doesn't return anything after channel creation
    /// Mock : The whole ChannelCreation<AppResponse> result is returned because the transactions and channel ids cannot be queried after
    type ChannelCreationResult;

    /// Error type for transactions in an environment.
    type Error: 'static + Into<InterchainError> + std::fmt::Debug + std::error::Error + Sync + Send;

    /// Returns a chain if it's registered in the environment
    /// Using `get_chain` to avoid confusions with the `Iterator::chain` function
    /// Returns an error if the provided chain doesn't exist
    /// The chain_id doesn't have to be the actual chain_id of the chain. The way this id is handled is implementation specific
    /// ``` rust
    /// use cw_orch::prelude::*;
    /// use cw_orch_interchain::prelude::*;
    /// use counter_contract::CounterContract;
    /// let interchain = MockBech32InterchainEnv::new(vec![("osmosis-1","osmo"),("archway-1","arch")]);
    ///
    /// let osmosis = interchain.get_chain("osmosis-1").unwrap();
    ///
    /// // The given chain object can be used directly with a contract instance for example
    /// let contract = CounterContract::new(osmosis);
    /// contract.upload().unwrap();
    ///
    /// ```
    fn get_chain(&self, chain_id: impl ToString) -> Result<Chain, Self::Error>;

    /// Returns every chain registered within the environment.
    /// To get a single chain, use [`InterchainEnv::get_chain`]
    /// ``` rust
    /// use cw_orch::prelude::*;
    /// use cw_orch_interchain::prelude::*;
    /// use counter_contract::CounterContract;
    /// let interchain = MockBech32InterchainEnv::new(vec![("osmosis-1","osmo"),("archway-1","arch")]);
    ///
    /// let all_chains: Vec<&MockBeck32> = interchain.chains().collect();
    ///
    /// ```
    fn chains<'a>(&'a self) -> impl Iterator<Item = &'a Chain>
    where
        Chain: 'a;

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
    /// In general, this function is not used outside of the trait implementation
    fn get_channel_creation_txs(
        &self,
        src_chain: ChainId,
        ibc_channel: &mut InterchainChannel<<Chain as IbcQueryHandler>::Handler>,
        channel_creation_result: Self::ChannelCreationResult,
    ) -> Result<ChannelCreationTransactionsResult<Chain>, Self::Error>;

    /// Creates a channel and returns the 4 transactions hashes for channel creation
    /// This function should be used in code to make sure the channel creation (+ eventual packet relaying) is awaited before continuing
    /// This shouldn't need to be re-implemented.
    /// ``` rust
    /// use cw_orch_interchain::prelude::*;
    /// let interchain = MockBech32InterchainEnv::new(vec![("osmosis-1","osmo"),("archway-1","arch")]);
    /// // This creates a channel between 2 chains to transfer tokens
    /// interchain.create_channel(
    ///     "osmosis-1",
    ///     "archway-1",
    ///     &PortId::transfer(),
    ///     &PortId::transfer(),
    ///     "ics20-1",
    ///     Some(cosmwasm_std::IbcOrder::Unordered),
    /// ).unwrap();
    /// ```
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
                chain: self.get_chain(src_chain).map_err(Into::into)?.ibc_handler(),
                chain_id: src_chain.to_string(),
                connection_id: Some(src_connection_id),
                port: src_port.clone(),
                channel: None,
            },
            IbcPort {
                chain: self.get_chain(dst_chain).map_err(Into::into)?.ibc_handler(),
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
            self.await_packets(src_chain, channel_creation_txs.init)
                .map_err(Into::into)?,
            self.await_packets(dst_chain, channel_creation_txs.r#try)
                .map_err(Into::into)?,
            self.await_packets(src_chain, channel_creation_txs.ack)
                .map_err(Into::into)?,
            self.await_packets(dst_chain, channel_creation_txs.confirm)
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

    /// This function creates a channel between 2 wasm contracts.
    ///
    /// This is a wrapper around [`Self::create_channel`] that gets its information from the contract objects
    /// ```rust,no_run
    /// use cw_orch_interchain::prelude::*;
    /// // Those 2 imports are solely used so that this doc compiles, the Host and Controller contract need to be defined
    /// // You can find the implementation here : https://github.com/confio/cw-ibc-demo/
    /// use counter_contract::CounterContract as Host;
    /// use counter_contract::CounterContract as Controller;
    ///
    /// let interchain = MockBech32InterchainEnv::new(vec![("osmosis-1","osmo"),("archway-1","arch")]);
    /// let osmosis = interchain.get_chain("osmosis-1").unwrap();
    /// let archway = interchain.get_chain("archway-1").unwrap();
    ///
    /// let ica_host = Host::new(osmosis);
    /// let ica_controller = Controller::new(archway);
    /// // This creates a channel between 2 chains to transfer tokens
    /// interchain.create_contract_channel(
    ///     &ica_controller,
    ///     &ica_host,
    ///     "simple-ica-v2",
    ///     Some(cosmwasm_std::IbcOrder::Unordered),
    /// ).unwrap();
    /// ```
    fn create_contract_channel(
        &self,
        src_contract: &dyn ContractInstance<Chain>,
        dst_contract: &dyn ContractInstance<Chain>,
        version: &str,
        order: Option<IbcOrder>,
    ) -> Result<ChannelCreationResult<Chain>, InterchainError> {
        let src_chain = src_contract.environment().chain_id();
        let dst_chain = dst_contract.environment().chain_id();

        let src_port = contract_port(src_contract);
        let dst_port = contract_port(dst_contract);

        let channel_creation =
            self.create_channel(&src_chain, &dst_chain, &src_port, &dst_port, version, order)?;

        Ok(channel_creation)
    }

    /// Follows every IBC packets sent out during a transaction
    /// This returns a packet analysis.
    ///
    /// For easier handling of the Interchain response, please use [`Self::await_and_check_packets`]
    /// ``` rust,no_run
    /// use cw_orch::prelude::*;
    /// use cw_orch_interchain::prelude::*;
    /// use ibc_proto::ibc::{
    ///         applications::transfer::v1::{MsgTransfer, MsgTransferResponse},
    ///         core::client::v1::Height,
    /// };
    /// use prost_types::Any;
    /// use cosmos_sdk_proto::{
    ///     traits::{Message, Name},
    /// };
    /// let starship = Starship::new(None).unwrap();
    /// let interchain = starship.interchain_env();
    /// // This creates a channel between 2 chains to transfer tokens
    /// let channel = interchain.create_channel(
    ///     "osmosis-1",
    ///     "archway-1",
    ///     &PortId::transfer(),
    ///     &PortId::transfer(),
    ///     "ics20-1",
    ///     Some(cosmwasm_std::IbcOrder::Unordered),
    /// ).unwrap();
    ///
    /// let src_channel = channel
    ///     .interchain_channel
    ///     .get_ordered_ports_from("osmosis-1").unwrap();
    ///
    /// let osmosis = interchain.get_chain("osmosis-1").unwrap();
    /// let archway = interchain.get_chain("archway-1").unwrap();
    /// let tx_resp = osmosis.commit_any::<MsgTransferResponse>(
    ///     vec![
    ///         Any {
    ///             value: MsgTransfer {
    ///                 source_port: src_channel.0.port.to_string(),
    ///                 source_channel: src_channel.0.channel.unwrap().to_string(),
    ///                 token: Some(ibc_proto::cosmos::base::v1beta1::Coin {
    ///                     amount: "100_000".to_string(),
    ///                     denom: "osmo".to_string(),
    ///                 }),
    ///                 sender: osmosis.sender_addr().to_string(),
    ///                 receiver: archway.sender_addr().to_string(),
    ///                 timeout_height: None,
    ///                 timeout_timestamp: osmosis.block_info().unwrap().time.plus_seconds(600).nanos(),
    ///                 memo: String::new(),
    ///             }.encode_to_vec(),
    ///             type_url: MsgTransfer::type_url(),
    ///         }
    ///     ],
    /// None,
    /// ).unwrap();
    ///
    /// // This simply checks that the packets have been successfully relayed but doesn't check wether the ack was a success
    /// interchain.await_packets("osmosis-1", tx_resp).unwrap();
    /// ```
    fn await_packets(
        &self,
        chain_id: ChainId,
        tx_response: <Chain as TxHandler>::Response,
    ) -> Result<IbcTxAnalysis<Chain>, Self::Error>;

    /// Follow every IBC packets sent out during the transaction
    /// Parses the acks according to usual ack formats (ICS20, Polytone, ICS-004)
    /// Errors if the acks and't be parsed, correspond to a failed result or there is a timeout
    /// If you only want to await without validation, use [`Self::await_packets`]
    ///
    /// ``` rust
    /// use cosmwasm_std::{coin, CosmosMsg, IbcMsg, IbcTimeout, IbcTimeoutBlock};
    /// use cw_orch::prelude::*;
    /// use cw_orch::mock::cw_multi_test::Executor;
    /// use cw_orch_interchain::prelude::*;
    /// use ibc_relayer_types::core::ics24_host::identifier::PortId;

    /// let interchain = MockInterchainEnv::new(vec![("juno-1", "sender"), ("stargaze-1", "sender")]);

    /// let channel = interchain.create_channel(
    ///     "juno-1",
    ///     "stargaze-1",
    ///     &PortId::transfer(),
    ///     &PortId::transfer(),
    ///     "ics20-1",
    ///     None,
    /// ).unwrap();
    /// let juno = interchain.get_chain("juno-1").unwrap();
    /// let stargaze = interchain.get_chain("stargaze-1").unwrap();

    /// let channel = channel
    ///     .interchain_channel
    ///     .get_ordered_ports_from("juno-1").unwrap();

    /// juno.add_balance(&juno.sender_addr(), vec![coin(100_000, "ujuno")]).unwrap();
    /// let tx_resp = juno.app.borrow_mut().execute(
    ///     juno.sender_addr(),
    ///     CosmosMsg::Ibc(IbcMsg::Transfer {
    ///         channel_id: channel.0.channel.unwrap().to_string(),
    ///         to_address: stargaze.sender_addr().to_string(),
    ///         amount: coin(100_000, "ujuno"),
    ///         timeout: IbcTimeout::with_block(IbcTimeoutBlock {
    ///             revision: 1,
    ///             height: stargaze.block_info().unwrap().height + 1,
    ///         }),
    ///         memo: None,
    ///     }),
    /// ).unwrap();

    /// // This makes sure that the packets arrive successfully and present a success ack
    /// let result = interchain.await_and_check_packets("juno-1", tx_resp).unwrap();
    /// ```    
    fn await_and_check_packets(
        &self,
        chain_id: ChainId,
        tx_response: <Chain as TxHandler>::Response,
    ) -> Result<(), InterchainError> {
        let tx_result = self
            .await_packets(chain_id, tx_response)
            .map_err(Into::into)?;

        tx_result.into_result()?;

        Ok(())
    }

    /// Follow the execution of a single IBC packet across the chain.
    /// It won't follow additional packets sent out during the transmission of this packet
    /// This is usually not used outside of the structure implementation, but is still available if needed
    fn await_single_packet(
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

    /// Returns all successful packets gathered during the packet following procedure
    /// Doesn't error if a packet has timed-out
    pub fn get_success_packets(&self) -> Result<Vec<SuccessIbcPacket<Chain>>, InterchainError> {
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
            IbcPacketOutcome::Timeout { .. } => Ok(vec![]),
        }
    }

    /// Returns all successful packets gathered during the packet following procedure
    /// Errors if a packet has timed-out
    pub fn assert_no_timeout(&self) -> Result<Vec<SuccessIbcPacket<Chain>>, InterchainError> {
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
                receive_tx.assert_no_timeout()?,
                ack_tx.assert_no_timeout()?,
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
