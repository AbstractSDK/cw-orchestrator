use std::{env, time::Duration};

use cosmos_sdk_proto::{cosmos::{auth::v1beta1::{query_client::{self, QueryClient},QueryAccountResponse, QueryAccountRequest}, bank::v1beta1::{msg_client::MsgClient}, tx::v1beta1::{BroadcastTxRequest, BroadcastMode}}};
use cosmrs::{Tx, Coin, tx::{SignerInfo, self, Body}, bank::MsgSend, AccountId};


use terra_rust_script::{
    helpers::get_configuration,
    traits::{CliInterface, Instance},
    chain::Chain,
};
use tonic::{transport::{Channel, Endpoint, Uri}, Response};

pub async fn script() -> anyhow::Result<()> {
    
    
        
        
        let grpc = env::var("LTERRA_GRPC").unwrap();
        let chain = Chain::new(
            "osanuth".into(),
            "juno".into(),
            grpc,
            118,
        ).await?;
        let (sender, config) = &get_configuration("uconst",chain).await?;
        let resp = sender.sequence_number().await?;
        log::debug!("{}", resp);
        log::debug!("{}",sender.pub_addr()?);

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
