
use tonic::transport::Channel;

use crate::error::TerraRustScriptError;

#[derive(Clone, Debug)]
pub struct Chain {
    pub chain_id: String,
    pub pub_addr_prefix: String,
    pub grpc_url: String,
    pub coin_type: u32,
    pub rpc_channel: Channel
}

impl Chain {
    pub async fn new(chain_id: String,
        pub_addr_prefix: String,
        grpc_url: String,
        coin_type: u32,) -> Result<Self, TerraRustScriptError> {

        let rpc_channel = Channel::from_shared(grpc_url.clone()).unwrap()
        .connect()
        .await?;

        Ok(Self{
            chain_id,
            coin_type,
            grpc_url,
            pub_addr_prefix,
            rpc_channel
        })
    }
}



