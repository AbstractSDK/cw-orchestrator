use cw_orch::{networks, GrpcChannel};

use ibc_chain_registry::chain::Grpc;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use speculoos::{asserting, result::ResultAssertions};

#[allow(unused)]
pub async fn build_channel() -> tonic::transport::Channel {
    let network = networks::LOCAL_JUNO;

    let grpcs: Vec<Grpc> = vec![Grpc {
        address: network.grpc_urls[0].into(),
        provider: None,
    }];

    let chain: ChainId = ChainId::new(network.chain_id.to_owned(), 1);

    let channel = GrpcChannel::connect(&grpcs, &chain).await;

    asserting!("channel connection is succesful")
        .that(&channel)
        .is_ok();

    channel.unwrap()
}
