use std::collections::HashMap;
use std::rc::Rc;
use cosmos_client::client::Rpc;
use cosmwasm_std::Addr;
use ibc_chain_registry::chain::ChainData;
use crate::error::CwOrchError;
use crate::prelude::StateInterface;
use crate::state::DeployDetails;

use super::error::WasmDaemonError;

/// Stores the chain information and deployment state.
/// Uses a simple JSON file to store the deployment information locally.
#[derive(Clone, Debug)]
pub struct WasmDaemonState {
    /// rpc url
    pub rpc_url: String,
    /// Information about the chain
    pub chain_data: ChainData,
}

impl WasmDaemonState {
    /// Creates a new state from the given chain data and deployment id.
    /// Attempts to connect to any of the provided gRPC endpoints.
    pub async fn new(
        mut chain_data: ChainData,
        deployment_id: String,
    ) -> Result<WasmDaemonState, WasmDaemonError> {
        if chain_data.apis.rpc.is_empty() {
            return Err(WasmDaemonError::RPCListIsEmpty);
        }

        log::info!("Found {} rpc endpoints", chain_data.apis.rpc.len());

        // Try to get the standard fee token (probably shortest denom)
        let shortest_denom_token = chain_data.fees.fee_tokens.iter().fold(
            chain_data.fees.fee_tokens[0].clone(),
            |acc, item| {
                if item.denom.len() < acc.denom.len() {
                    item.clone()
                } else {
                    acc
                }
            },
        );
        // set a single fee token
        chain_data.fees.fee_tokens = vec![shortest_denom_token];

        // build daemon state
        let state = WasmDaemonState {
            rpc_url: chain_data.apis.rpc.get(0).unwrap().address.clone(),
            chain_data,
        };

        // finish
        Ok(state)
    }
}


impl StateInterface for Rc<WasmDaemonState> {
    fn get_address(&self, contract_id: &str) -> Result<Addr, CwOrchError> {
        todo!()
    }

    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        todo!()
    }

    fn get_code_id(&self, contract_id: &str) -> Result<u64, CwOrchError> {
        todo!()
    }

    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        todo!()
    }

    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, CwOrchError> {
        todo!()
    }

    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, CwOrchError> {
        todo!()
    }

    fn deploy_details(&self) -> DeployDetails {
        todo!()
    }
}
