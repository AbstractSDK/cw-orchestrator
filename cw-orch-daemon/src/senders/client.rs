use std::future::Future;

use crate::error::DaemonError;
use cw_orch_core::environment::ChainInfoOwned;
/// A client for sending messages to a CosmosSDK chain.
/// If the `grpc` feature is enabled, the client will use gRPC to send messages.
/// If the `rpc` feature is enabled, the client will use HTTP to send messages.
#[derive(Debug, Clone)]
pub struct CosmosClient {
    // TODO: if both enabled, default to RPC
    #[cfg(feature = "grpc")]
    inner: tonic::transport::Channel,
    #[cfg(feature = "rpc")]
    inner: HttpClient,
}

impl CosmosClient {
    /// Create a gRPC channel from the chain info
    pub async fn from_chain_info(chain_info: &ChainInfoOwned) -> Result<Self, DaemonError> {
        // RPC overwrites gRPC if both features are enabled
        #[cfg(feature = "rpc")]
        if cfg!(feature = "rpc") {
            return Ok(Self {
                inner: self::rpc::RpcChannel::connect(&chain_info.grpc_urls, &chain_info.chain_id)
                    .await?,
            });
        }

        #[cfg(feature = "grpc")]
        Ok(Self {
            inner: self::grpc::GrpcChannel::connect(
                &chain_info.grpc_urls,
                &chain_info.chain_id,
            )
            .await?,
        })
    }
}

trait Connect {
    type Connection;
    /// Connect to an endpoint and return a client
    fn connect(
        urls: &[String],
        chain_id: &str,
    ) -> impl Future<Output = Result<Self::Connection, DaemonError>> + Send;
}

mod rpc {
    use cosmrs::rpc::{Client, HttpClient};

    use crate::error::DaemonError;

    use super::Connect;

    /// A helper for constructing a gRPC channel
    pub struct RpcChannel {}

    impl Connect for RpcChannel {
        type Connection = HttpClient;

        /// Connect to any of the provided gRPC endpoints
        async fn connect(urls: &[String], chain_id: &str) -> Result<HttpClient, DaemonError> {
            let mut successful_connections = vec![];

            for address in urls.iter() {
                log::info!("Trying to connect to endpoint: {}", address);

                // try to connect to rpc endpoint
                let maybe_client = HttpClient::new(address.as_str());

                // connection succeeded or try the next rpc endpoint
                let client = if maybe_client.is_ok() {
                    maybe_client?
                } else {
                    continue;
                };

                // get client information for verification down below
                let node_info = client.status().await?.node_info;

                // verify we are connected to the spected network
                if node_info.network.as_str().ne(chain_id) {
                    log::error!(
                        "Network mismatch: connection:{} != config:{}",
                        node_info.network,
                        chain_id
                    );
                    continue;
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

            let build_res = DaemonAsync::builder(chain)
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

            let build_res = DaemonAsync::builder(chain)
                .deployment_id("v0.1.0")
                .build()
                .await;

            asserting!("GRPC list is empty")
                .that(&build_res.err().unwrap().to_string())
                .is_equal_to(String::from("The list of grpc endpoints is empty"))
        }
    }
}

mod grpc {
    use cosmrs::proto::cosmos::base::tendermint::v1beta1::{
        service_client::ServiceClient, GetNodeInfoRequest,
    };
    use cw_orch_core::{environment::ChainInfoOwned, log::connectivity_target};
    use tonic::transport::{Channel, ClientTlsConfig};

    use crate::error::DaemonError;

    use super::Connect;

    /// A helper for constructing a gRPC channel
    pub struct GrpcChannel {}

    impl Connect for GrpcChannel {
        type Connection = Channel;

        /// Connect to any of the provided gRPC endpoints
        async fn connect(grpc: &[String], chain_id: &str) -> Result<Channel, DaemonError> {
            if grpc.is_empty() {
                return Err(DaemonError::GRPCListIsEmpty);
            }

            let mut successful_connections = vec![];

            for address in grpc.iter() {
                log::debug!(target: &connectivity_target(), "Trying to connect to endpoint: {}", address);

                // get grpc endpoint
                let endpoint = Channel::builder(address.clone().try_into().unwrap());

                // try to connect to grpc endpoint
                let maybe_client = ServiceClient::connect(endpoint.clone()).await;

                // connection succeeded
                let mut client = if maybe_client.is_ok() {
                    maybe_client?
                } else {
                    log::warn!(
                        "Cannot connect to gRPC endpoint: {}, {:?}",
                        address,
                        maybe_client.unwrap_err()
                    );

                    // try HTTPS approach
                    // https://github.com/hyperium/tonic/issues/363#issuecomment-638545965
                    if !(address.contains("https") || address.contains("443")) {
                        continue;
                    };

                    log::debug!(target: &connectivity_target(), "Attempting to connect with TLS");

                    // re attempt to connect
                    let endpoint = endpoint.clone().tls_config(ClientTlsConfig::new())?;
                    let maybe_client = ServiceClient::connect(endpoint.clone()).await;

                    // connection still fails
                    if maybe_client.is_err() {
                        log::warn!(
                            "Cannot connect to gRPC endpoint: {}, {:?}",
                            address,
                            maybe_client.unwrap_err()
                        );
                        continue;
                    };

                    maybe_client?
                };

                // get client information for verification down below
                let node_info = client
                    .get_node_info(GetNodeInfoRequest {})
                    .await?
                    .into_inner();

                // local juno does not return a proper ChainId with epoch format
                // verify we are connected to the expected network
                if node_info.default_node_info.as_ref().unwrap().network != chain_id {
                    log::error!(
                        "Network mismatch: connection:{} != config:{}",
                        node_info.default_node_info.as_ref().unwrap().network,
                        chain_id
                    );
                    continue;
                }

                // add endpoint to succesful connections
                successful_connections.push(endpoint.connect().await?)
            }

            // we could not get any succesful connections
            if successful_connections.is_empty() {
                return Err(DaemonError::CannotConnectGRPC);
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
        #[serial_test::serial]
        async fn no_connection() {
            let mut chain = cw_orch_daemon::networks::LOCAL_JUNO;
            let grpcs = &["https://127.0.0.1:99999"];
            chain.grpc_urls = grpcs;

            let build_res = DaemonAsync::builder(chain)
                .deployment_id("v0.1.0")
                .build_sender(())
                .await;

            asserting!("there is no GRPC connection")
                .that(&build_res.err().unwrap().to_string())
                .is_equal_to(String::from(
                    "Can not connect to any grpc endpoint that was provided.",
                ))
        }

        #[tokio::test]
        #[serial_test::serial]
        async fn network_grpcs_list_is_empty() {
            let mut chain = cw_orch_daemon::networks::LOCAL_JUNO;
            let grpcs = &[];
            chain.grpc_urls = grpcs;

            let build_res = DaemonAsync::builder(chain)
                .deployment_id("v0.1.0")
                .build_sender(())
                .await;

            asserting!("GRPC list is empty")
                .that(&build_res.err().unwrap().to_string())
                .is_equal_to(String::from("The list of grpc endpoints is empty"))
        }
    }
}
