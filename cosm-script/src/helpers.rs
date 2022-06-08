use std::{env, rc::Rc};

use secp256k1::{All, Secp256k1};

use crate::{
    Deployment,
    Chain, Network,
    sender::Sender,
};

pub fn get_env_vars() -> (String, String, String, bool) {
    let propose_on_multisig = env::var("PROPOSE_ON_MULTISIG").unwrap_or("false".to_string());
    let addr_path = env::var("ADDRESS_JSON").unwrap();
    let group = env::var("DEPLOYMENT").unwrap();
    let chain = env::var("CHAIN").unwrap();

    (
        addr_path,
        chain,
        group,
        propose_on_multisig.parse::<bool>().unwrap(),
    )
}

pub async fn get_configuration() -> anyhow::Result<(Deployment, Rc<Sender<All>>)> {
    let secp = Secp256k1::new();
    let (addr_path, chain_id, deployment_id, propose_on_multisig) = get_env_vars();

    let chain = Chain::get(&chain_id).await?;
    let network = chain.network().await?;
    // All configs are set here
    let config = Deployment::new(deployment_id, network, addr_path, propose_on_multisig).await?;

    let sender = Rc::new(Sender::new(config.clone(), secp)?);
    Ok((config, sender))
}
