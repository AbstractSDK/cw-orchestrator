use cw_orch_core::environment::TxHandler;
use prost::Message;
use prost_types::Any;

/// Alows the execution of stargate like messages on cw-orch environments
pub trait Stargate: TxHandler {
    /// Execute a custom abci/starship message on the environment
    /// The message should already be encoded as protobuf any
    /// We enforce a generic parameter to be able to unwrap the response (because of osmosis test tube for now)
    /// This is however always a good practice to know which is the response to the tx type that you send
    fn commit_any<R: Message + Default>(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> Result<<Self as TxHandler>::Response, <Self as TxHandler>::Error>;
}
