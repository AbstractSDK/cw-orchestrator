use super::error::DaemonError;
use crate::{chain_info::ChainKind, channel::GrpcChannel};

use ibc_chain_registry::chain::ChainData;
use serde::Serialize;
use serde_json::{json, Value};
use std::{env, fs::File, path::Path};
use tonic::transport::Channel;

/// Stores the chain information and deployment state.
/// Uses a simple JSON file to store the deployment information locally.
#[derive(Clone, Debug)]
pub struct DaemonState {
    /// this is passed via env var STATE_FILE
    pub json_file_path: String,
    /// Deployment identifier
    pub deployment_id: String,
    /// gRPC channel
    pub grpc_channel: Channel,
    /// Information about the chain
    pub chain_data: ChainData,
}

impl DaemonState {
    /// Creates a new state from the given chain data and deployment id.
    /// Attempts to connect to any of the provided gRPC endpoints.
    pub async fn new(
        mut chain_data: ChainData,
        deployment_id: String,
    ) -> Result<DaemonState, DaemonError> {
        if chain_data.apis.grpc.is_empty() {
            return Err(DaemonError::GRPCListIsEmpty);
        }

        log::info!("Found {} gRPC endpoints", chain_data.apis.grpc.len());

        // find working grpc channel
        let grpc_channel =
            GrpcChannel::connect(&chain_data.apis.grpc, &chain_data.chain_id).await?;

        // check if STATE_FILE en var is configured, default to state.json
        let mut json_file_path = env::var("STATE_FILE").unwrap_or("./state.json".to_string());

        // if the network we are connecting is a local kind, add it to the fn
        if chain_data.network_type == ChainKind::Local.to_string() {
            let name = Path::new(&json_file_path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let folder = Path::new(&json_file_path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap();

            json_file_path = format!("{folder}/{name}_local.json");
        }

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
        let state = DaemonState {
            json_file_path,
            deployment_id,
            grpc_channel,
            chain_data,
        };

        log::info!(
            "Writing daemon state JSON file: {:#?}",
            state.json_file_path
        );

        // write json state file
        crate::json_file::write(
            &state.json_file_path,
            &state.chain_data.chain_id.to_string(),
            &state.chain_data.chain_name,
            &state.deployment_id,
        );

        // finish
        Ok(state)
    }

    /// Get the state filepath and read it as json
    fn read_state(&self) -> serde_json::Value {
        crate::json_file::read(&self.json_file_path)
    }

    /// Retrieve a stateful value using the chainId and networkId
    pub fn get(&self, key: &str) -> Value {
        let json = self.read_state();
        json[&self.chain_data.chain_name][&self.chain_data.chain_id.to_string()][key].clone()
    }

    /// Set a stateful value using the chainId and networkId
    pub fn set<T: Serialize>(&self, key: &str, contract_id: &str, value: T) {
        let mut json = self.read_state();

        json[&self.chain_data.chain_name][&self.chain_data.chain_id.to_string()][key]
            [contract_id] = json!(value);

        serde_json::to_writer_pretty(File::create(&self.json_file_path).unwrap(), &json).unwrap();
    }
}
