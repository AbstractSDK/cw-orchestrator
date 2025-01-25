use std::marker::PhantomData;

use cosmwasm_std::Empty;
use cw_orch_core::environment::{CwEnv, IndexResponse, TxHandler};

/// Identifies a transaction
#[derive(Clone)]
pub struct TxId<Chain: CwEnv, CustomResult = Empty> {
    /// Chain Id on which the transaction was broadcasted
    pub chain_id: String,
    /// Transactions response for the transaction (env specific)
    pub response: <Chain as TxHandler>::Response,

    _phantom_data: PhantomData<CustomResult>,
}

impl<Chain: CwEnv, CustomResult> TxId<Chain, CustomResult> {
    /// Creates a new Tx Id object identifying a transaction exactly
    pub fn new(chain_id: String, response: <Chain as TxHandler>::Response) -> Self {
        TxId {
            chain_id,
            response,
            _phantom_data: Default::default(),
        }
    }
}

impl<C: CwEnv, CustomResult: std::fmt::Debug> std::fmt::Debug for TxId<C, CustomResult> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TxId")
            .field("chain_id", &self.chain_id)
            .field("response", &self.response)
            .finish()
    }
}

impl<Chain: CwEnv, CustomResult> IndexResponse for TxId<Chain, CustomResult> {
    fn events(&self) -> Vec<cosmwasm_std::Event> {
        self.response.events()
    }

    fn event_attr_value(
        &self,
        event_type: &str,
        attr_key: &str,
    ) -> cosmwasm_std::StdResult<String> {
        self.response.event_attr_value(event_type, attr_key)
    }

    fn event_attr_values(&self, event_type: &str, attr_key: &str) -> Vec<String> {
        self.response.event_attr_values(event_type, attr_key)
    }

    fn data(&self) -> Option<cosmwasm_std::Binary> {
        self.response.data()
    }
}
