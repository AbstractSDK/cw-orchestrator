pub const MAX_TX_QUERY_RETRIES: usize = 50;

#[cfg(feature = "rpc")]
pub mod rpc;
#[cfg(feature = "rpc")]
pub use rpc::*;
#[cfg(feature = "grpc")]
pub mod grpc;
#[cfg(feature = "grpc")]
pub use grpc::*;

/// Constructor for a querier over a given channel
pub trait DaemonQuerier {
    /// Construct an new querier over a given channel
    #[cfg(feature = "rpc")]
    fn new(client: cosmrs::rpc::HttpClient) -> Self;
    #[cfg(feature = "grpc")]
    fn new(channel: tonic::transport::Channel) -> Self;
}
