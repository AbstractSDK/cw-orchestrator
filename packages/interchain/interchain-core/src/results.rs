use crate::ibc_query::IbcQueryHandler;
use crate::packet::NestedPacketsFlow;
use crate::{channel::InterchainChannel, env::ChannelCreation};
use cw_orch_core::environment::TxHandler;
use ibc_relayer_types::core::ics24_host::identifier::ChannelId;

/// Chain identification for cw-orch Ibc purposes
pub type NetworkId = String;

// Return types for the env trait
/// Result returned by  InterchainEnv::_internal_create_channel
pub struct InternalChannelCreationResult<ChannelCreationResult> {
    /// Channel creation result specific the used chain
    pub result: ChannelCreationResult,
    /// Connection id on which the channel was created.
    /// This connection id is supposed to be known by the channel creation environment
    pub src_connection_id: String,
}

/// Result returned by  InterchainEnv::get_channel_creation_txs
pub struct ChannelCreationTransactionsResult<Chain: TxHandler> {
    /// Id of the channel that was just created on the src chain
    pub src_channel_id: ChannelId,
    /// Id of the channel that was just created on the dst chain
    pub dst_channel_id: ChannelId,
    /// Transactions involved in the channel creation
    pub channel_creation_txs: ChannelCreation<<Chain as TxHandler>::Response>,
}

/// Result returned by  InterchainEnv::create_channel
pub struct ChannelCreationResult<Chain: IbcQueryHandler> {
    /// Channel object containing every variable needed for identifying the channel that was just created
    pub interchain_channel: InterchainChannel<<Chain as IbcQueryHandler>::Handler>,
    /// Transactions involved in the channel creation + Their packet following analysis
    pub channel_creation_txs: ChannelCreation<NestedPacketsFlow<Chain>>,
}
