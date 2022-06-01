use std::{env, rc::Rc};

use secp256k1::{All, Secp256k1};

use crate::{sender::{GroupConfig, Network, Sender}, chain::Chain};

pub fn get_env_vars() -> (String, String, Network, bool) {
    let propose_on_multisig = env::var("PROPOSE_ON_MULTISIG").unwrap_or("false".to_string());
    let path = env::var("ADDRESS_JSON").unwrap();
    let group = env::var("GROUP").unwrap();

    let network = match group.as_str() {
        "testnet" => Network::Testnet,
        "mainnet" => Network::Mainnet,
        _ => Network::LocalTerra,
    };
    (
        path,
        group,
        network,
        propose_on_multisig.parse::<bool>().unwrap(),
    )
}

pub async fn get_configuration(fee_denom: &str, chain: Chain) -> anyhow::Result<(Rc<Sender<All>>, GroupConfig)> {
    let secp = Secp256k1::new();
    let client = reqwest::Client::new();
    let (path, group_name, network, propose_on_multisig) = get_env_vars();

    // All configs are set here
    let config = GroupConfig::new(
        network,
        group_name,
        client,
        fee_denom,
        path,
        propose_on_multisig,
    )
    .await?;

    let sender = Rc::new(Sender::new(&config, chain,secp)?);
    Ok((sender, config))
}
