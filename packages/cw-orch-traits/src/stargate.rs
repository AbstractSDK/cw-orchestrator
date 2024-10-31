use cw_orch_core::environment::TxHandler;
use prost_types::Any;

/// Alows the execution of stargate like messages on cw-orch environments
pub trait Stargate: TxHandler {
    /// Execute a custom abci/starship message on the environment
    /// The message should already be encoded as protobuf any
    fn commit_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> Result<<Self as TxHandler>::Response, <Self as TxHandler>::Error>;
}
