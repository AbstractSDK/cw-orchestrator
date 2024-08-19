use cosmos_sdk_proto::traits::{Message, Name};
use cw_orch::{environment::QueryHandler, prelude::*};
use cw_orch_interchain_core::InterchainEnv;
use cw_orch_interchain_daemon::ChannelCreator as _;
use cw_orch_starship::Starship;
use ibc_proto::ibc::{
    applications::transfer::v1::{MsgTransfer, MsgTransferResponse},
    core::client::v1::Height,
};
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use prost_types::Any;
fn main() -> cw_orch::anyhow::Result<()> {
    pretty_env_logger::init();

    let starship = Starship::new(None)?;
    let interchain = starship.interchain_env();

    let channel = interchain.create_channel(
        "juno-1",
        "stargaze-1",
        &PortId::transfer(),
        &PortId::transfer(),
        "ics20-1",
        None,
    )?;

    // We send an invalid ICs20 packet to the chains which will timeout soon.
    // We see what the relayer does and returns a timeout successfully
    let juno = starship.daemon("juno-1")?;
    let stargaze = starship.daemon("stargaze-1")?;
    let stargaze_height = stargaze.block_info()?;
    let channel = channel
        .interchain_channel
        .get_ordered_ports_from("juno-1")?;

    let tx_resp = juno.commit_any::<MsgTransferResponse>(
        vec![Any {
            value: MsgTransfer {
                source_port: channel.0.port.to_string(),
                source_channel: channel.0.channel.unwrap().to_string(),
                token: Some(ibc_proto::cosmos::base::v1beta1::Coin {
                    amount: "100_000".to_string(),
                    denom: "ujuno".to_string(),
                }),
                sender: juno.sender_addr().to_string(),
                receiver: stargaze.sender_addr().to_string(),
                timeout_height: Some(Height {
                    revision_number: 1,
                    revision_height: stargaze_height.height,
                }),
                timeout_timestamp: 0,
                memo: String::new(),
            }
            .encode_to_vec(),
            type_url: MsgTransfer::type_url(),
        }],
        None,
    )?;

    let result = interchain.await_packets("juno-1", tx_resp)?;

    match &result.packets[0].outcome {
        cw_orch_interchain_core::types::IbcPacketOutcome::Timeout { .. } => {}
        cw_orch_interchain_core::types::IbcPacketOutcome::Success { .. } => {
            panic!("Expected timeout")
        }
    }

    Ok(())
}
