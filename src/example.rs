use std::env;

use crate::{
    contract_instances::memory::Memory,
    contract_instances::os_factory::OsFactory,
    sender::{GroupConfig, Network, Sender},
};
use pandora_os::governance::gov_type::GovernanceDetails;
use secp256k1::Secp256k1;

use pandora_os::memory::msg::{
    ExecuteMsg as MemExec, InstantiateMsg as MemInit, QueryMsg as MemQuery,
};
use pandora_os::os_factory::msg::{
    ExecuteMsg as OsFactExec, InstantiateMsg as OsFactInit, QueryMsg as OsFactQuery,
};

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
    let sender = &Sender::new(&config, secp)?;

    let memory = Memory::new(config.clone());
    // memory.save_contract_address("terra1x7r06zec02mermgegpu56u8pjdtfekhx2sz54s".to_string())?;

    // let os_factory = OsFactory::new(config.clone());
    memory
        .execute(
            sender,
            MemExec::update_asset_addresses(vec![], vec![]),
            vec![],
        )
        .await?;

    // let governance = GovernanceDetails::Monarchy {
    //     monarch: sender.pub_addr()?
    // };
    // os_factory.execute(sender, OsFactExec::create_os(governance), vec![]).await?;

    // memory.0.upload(&sender, "/home/cyberhoward/Programming/Pandora/contracts/artifacts/memory.wasm").await?;

    Ok(())
}
