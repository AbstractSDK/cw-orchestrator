use std::{env, rc::Rc};

use secp256k1::{All, Secp256k1};

use crate::{sender::Sender, Chain, Deployment};

pub fn get_env_vars() -> (String, String, String, bool) {
    let propose_on_multisig =
        env::var("PROPOSE_ON_MULTISIG").unwrap_or_else(|_| "false".to_string());
    let store_path = env::var("STORE").unwrap();
    let chain = env::var("CHAIN").unwrap();
    let deployment = env::var("DEPLOYMENT").unwrap();

    (
        store_path,
        chain,
        deployment,
        propose_on_multisig.parse::<bool>().unwrap(),
    )
}

pub async fn get_configuration() -> anyhow::Result<(Deployment, Rc<Sender<All>>)> {
    let secp = Secp256k1::new();
    let (store_path, chain_id, deployment_id, propose_on_multisig) = get_env_vars();

    let chain = Chain::new(&chain_id, &store_path).await?;
    let network = chain.network().await?;
    // All configs are set here
    let config = Deployment::new(deployment_id, network, propose_on_multisig).await?;

    let sender = Rc::new(Sender::new(config.clone(), secp)?);
    Ok((config, sender))
}
