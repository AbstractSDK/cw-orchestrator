/*
*/
#[cfg(test)]
mod querier {
    use speculoos::prelude::*;

    use std::sync::Arc;
    use tokio::runtime::Runtime;

    use crate::{
        daemon::channel::DaemonChannel,
        daemon::networks,
        daemon::querier::DaemonQuerier,
        daemon::state::{DaemonOptions, DaemonOptionsBuilder},
    };

    async fn build_channel() -> Option<tonic::transport::Channel> {
        let options: DaemonOptions = DaemonOptionsBuilder::default()
            .network(networks::LOCAL_JUNO)
            .deployment_id("v0.1.0")
            .build()
            .unwrap();

        let network = options.get_network();

        let channel = DaemonChannel::connect(&network.apis.grpc, &network.chain_id)
            .await
            .unwrap();

        asserting!("channel is some").that(&channel).is_some();

        channel
    }

    #[test]
    fn general() {
        let rt = Arc::new(Runtime::new().unwrap());

        let channel = rt.block_on(build_channel()).unwrap();

        let block_height = rt.block_on(DaemonQuerier::block_height(channel.clone()));
        asserting!("block_height is ok").that(&block_height).is_ok();

        let latest_block = rt.block_on(DaemonQuerier::latest_block(channel.clone()));
        asserting!("latest_block is ok").that(&latest_block).is_ok();

        let block_time = rt.block_on(DaemonQuerier::block_time(channel.clone()));
        asserting!("block_time is ok").that(&block_time).is_ok();

        // NOTE: this needs a live contract
        // let contract_info = DaemonQuerier::contract_info(channel.clone().unwrap())
        //     .await
        //     .unwrap();
        // println!("contract_info: {:#?}", contract_info);

        // NOTE: this needs creating a correct tx operation
        // let exec_msg = cw20_base::msg::ExecuteMsg::Mint {
        //     recipient: "cosmos789".into(),
        //     amount: 128u128.into(),
        // };

        // let exec_msg: MsgExecuteContract = MsgExecuteContract {
        //     sender: AccountId::from_str("cosmos12345").unwrap(),
        //     contract: AccountId::from_str("contract12345").unwrap(),
        //     msg: serde_json::to_vec(&exec_msg).unwrap(),
        //     funds: parse_cw_coins(&vec![]).unwrap(),
        // };

        // missing lots of steps here... ooff

        // let simulate_tx = DaemonQuerier::simulate_tx(
        //     channel.clone().unwrap(),
        //     String::into_bytes(String::from("something")),
        // )
        // .await
        // .unwrap();
        // println!("simulate_tx: {:#?}", simulate_tx);
    }
}
