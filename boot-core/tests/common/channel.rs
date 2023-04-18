use boot_core::{networks, DaemonChannel, DaemonOptions, DaemonOptionsBuilder};

use speculoos::{asserting, prelude::OptionAssertions};

#[allow(unused)]
pub async fn build_channel() -> Option<tonic::transport::Channel> {
    let options: DaemonOptions = DaemonOptionsBuilder::default()
        .network(networks::LOCAL_JUNO)
        .deployment_id("v0.1.0")
        .build()
        .unwrap();

    let network = options.get_network();

    let channel = DaemonChannel::connect(&network.apis.grpc, &network.chain_id)
        .await
        .unwrap();

    asserting!("channel connection is succesful")
        .that(&channel)
        .is_some();

    channel
}
