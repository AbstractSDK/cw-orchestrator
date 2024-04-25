use cw_orch_core::environment::DefaultQueriers;
use cw_orch_core::environment::NodeQuerier;
use cw_orch_daemon::DaemonBuilder;
use cw_orch_networks::networks;

// From https://github.com/CosmosContracts/juno/blob/32568dba828ff7783aea8cb5bb4b8b5832888255/docker/test-user.env#L2
const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";
const TX_HASH: &str = "501DB67945FCBDFF3248F1C18A721E5D107D862A8F77045D88FF2ECC7924C65A";
pub fn main() -> anyhow::Result<()> {
    std::env::set_var("MAIN_MNEMONIC", LOCAL_MNEMONIC);
    env_logger::init();

    let mut network = networks::OSMOSIS_1;
    network.grpc_urls = &["https://osmosis.grpc.stakin-nodes.com:443"];
    let daemon = DaemonBuilder::default().chain(network).build()?;

    let tx = daemon.node_querier().find_tx(TX_HASH.to_string())?;

    let log_events = tx.get_events_from_logs("transfer");
    let events = tx.get_events("transfer");

    println!("Log events : {:?}, events : {:?}", log_events, events);

    Ok(())
}
