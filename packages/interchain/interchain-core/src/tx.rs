use cw_orch_core::environment::{CwEnv, TxHandler};


/// Identifies a transaction
#[derive(Clone)]
pub struct TxId<Chain: CwEnv> {
    /// Chain Id on which the transaction was broadcasted
    pub chain_id: String,
    /// Transactions response for the transaction (env specific)
    pub response: <Chain as TxHandler>::Response,
}

impl<C: CwEnv> std::fmt::Debug for TxId<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TxId")
            .field("chain_id", &self.chain_id)
            .field("response", &self.response)
            .finish()
    }
}