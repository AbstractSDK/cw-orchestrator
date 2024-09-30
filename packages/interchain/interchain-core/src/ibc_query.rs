use crate::results::NetworkId;
use crate::analysis::IbcTxAnalysis;
use crate::{channel::InterchainChannel, env::ChannelCreation};
use cosmwasm_std::{Api, Binary, StdError};
use cw_orch_core::environment::IndexResponse;
use cw_orch_core::environment::QueryHandler;
use cw_orch_core::environment::{CwEnv, TxHandler};
use cw_orch_mock::{MockBase, MockState};
use ibc_relayer_types::core::{
    ics04_channel::packet::Sequence,
    ics24_host::identifier::{ChannelId, PortId},
};

/// Adds additional capabilities to CwEnv for use with ibc environments
pub trait IbcQueryHandler: CwEnv {
    /// Query handler for the environment
    /// This should allow users to query anything related to IBC functionalities on the environment (if possible)
    type Handler: Clone + Send + Sync;

    /// Returns the `IbcQueryHandler::Handler` associated with the environment
    fn ibc_handler(&self) -> Self::Handler;

    /// Returns the chain id of the environment (for ibc identification purposes)
    fn chain_id(&self) -> NetworkId;
}

#[cfg(feature = "daemon")]
// Temporary until we can actually push to cw-orch-daemon
impl IbcQueryHandler for cw_orch_daemon::Daemon {
    type Handler = tonic::transport::Channel;

    fn ibc_handler(&self) -> tonic::transport::Channel {
        self.channel()
    }

    fn chain_id(&self) -> NetworkId {
        use cw_orch_core::environment::ChainState;

        self.state().chain_data.chain_id.to_string()
    }
}

// Temporary until we can actually push to cw-orch-mock
impl<A: Api> IbcQueryHandler for MockBase<A, MockState> {
    type Handler = ();
    fn ibc_handler(&self) {}

    fn chain_id(&self) -> NetworkId {
        self.block_info().unwrap().chain_id
    }
}