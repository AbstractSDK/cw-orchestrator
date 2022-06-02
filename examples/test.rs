use std::{env, time::Duration};

use cosm_rust_script::helpers::get_configuration;
use cosmos_sdk_proto::cosmos::{
    auth::v1beta1::{
        query_client::{self, QueryClient},
        QueryAccountRequest, QueryAccountResponse,
    },
    bank::v1beta1::msg_client::MsgClient,
    tx::v1beta1::{BroadcastMode, BroadcastTxRequest},
};
use cosmrs::{
    bank::MsgSend,
    tx::{self, Body, SignerInfo},
    AccountId, Coin, Tx,
};

use tonic::{
    transport::{Channel, Endpoint, Uri},
    Response,
};

pub async fn script() -> anyhow::Result<()> {
    let (config, sender) = get_configuration().await?;

    let amount = Coin {
        amount: 1u8.into(),
        denom: "ustake".parse().unwrap(),
    };

    let resp = sender
        .bank_send("juno1snm7svvfg85trrvzhphkgjc4tqwafad9devy6p", vec![amount])
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = script().await {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}
