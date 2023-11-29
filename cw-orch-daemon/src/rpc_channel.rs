use cosmrs::rpc::{HttpClient, Client};
use ibc_chain_registry::chain::Rpc;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use super::error::DaemonError;

/// A helper for constructing a gRPC channel
pub struct RpcChannel {}

impl RpcChannel {
    /// Connect to any of the provided gRPC endpoints
    pub async fn connect(rpc: &[Rpc], chain_id: &ChainId) -> Result<HttpClient, DaemonError> {
        let mut successful_connections = vec![];

        for Rpc { address, .. } in rpc.iter() {
            log::info!("Trying to connect to endpoint: {}", address);

            // try to connect to rpc endpoint
            let maybe_client = HttpClient::new(address.as_str());

            // connection succeeded or try the next rpc endpoint
            let client = if maybe_client.is_ok() {
                maybe_client?
            } else {
                continue
            };

            // get client information for verification down below
            let node_info = client
                .status().await?
                .node_info;  

            // local juno does not return a proper ChainId with epoch format
            if ChainId::is_epoch_format(node_info.network.as_str()) {
                // verify we are connected to the spected network
                if node_info.network.as_str().ne(chain_id.as_str()) {
                    log::error!(
                        "Network mismatch: connection:{} != config:{}",
                        node_info.network,
                        chain_id.as_str()
                    );
                    continue;
                }
            }

            // add endpoint to succesful connections
            successful_connections.push(client)
        }

        // we could not get any succesful connections
        if successful_connections.is_empty() {
            return Err(DaemonError::CannotConnectRPC);
        }

        Ok(successful_connections.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    /*
        This test asserts breaking issues around the GRPC connection
    */

    use crate::DaemonAsync;
    use speculoos::prelude::*;

    #[tokio::test]
    async fn no_connection() {
        let mut chain = cw_orch_daemon::networks::LOCAL_JUNO;
        let grpcs = &vec!["https://127.0.0.1:99999"];
        chain.grpc_urls = grpcs;

        let build_res = DaemonAsync::builder()
            .chain(chain)
            .deployment_id("v0.1.0")
            .build()
            .await;

        asserting!("there is no GRPC connection")
            .that(&build_res.err().unwrap().to_string())
            .is_equal_to(String::from(
                "Can not connect to any grpc endpoint that was provided.",
            ))
    }

    #[tokio::test]
    async fn network_grpcs_list_is_empty() {
        let mut chain = cw_orch_daemon::networks::LOCAL_JUNO;
        let grpcs: &Vec<&str> = &vec![];
        chain.grpc_urls = grpcs;

        let build_res = DaemonAsync::builder()
            .chain(chain)
            .deployment_id("v0.1.0")
            .build()
            .await;

        asserting!("GRPC list is empty")
            .that(&build_res.err().unwrap().to_string())
            .is_equal_to(String::from("The list of grpc endpoints is empty"))
    }
}
