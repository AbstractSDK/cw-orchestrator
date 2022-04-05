use std::env;

use crate::sender::{GroupConfig, Network, Sender};

use secp256k1::Secp256k1;

pub async fn demo() -> anyhow::Result<()> {
    let secp = Secp256k1::new();
    let client = reqwest::Client::new();
    let path = env::var("ADDRESS_JSON")?;
    let propose_on_multisig = true;

    // All configs are set here
    let group_name = "debugging".to_string();
    let config = GroupConfig::new(
        Network::Testnet,
        group_name,
        client,
        "uusd",
        path,
        propose_on_multisig,
        &secp,
    )
    .await?;
    let _sender = &Sender::new(&config, secp)?;

    // write custom logic

    Ok(())
}
