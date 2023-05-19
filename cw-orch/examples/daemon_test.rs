use std::env;

use contract_counter::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use cw_orch::prelude::{
    networks, ContractInstance, CwOrcExecute, CwOrcInstantiate, CwOrcQuery, CwOrcUpload, Daemon,
    TxHandler,
};
use tokio::runtime::Runtime;

extern crate cw_orch;
mod counter_contract;

const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

pub fn main() {
    let runtime = Runtime::new().unwrap();

    let res = Daemon::builder()
        .chain(networks::LOCAL_JUNO)
        .handle(runtime.handle())
        .mnemonic(LOCAL_MNEMONIC)
        .build();

    let Some(daemon) = res.as_ref().ok() else {
        panic!("Error: {}", res.err().unwrap());
    };

    let contract_counter =
        contract_counter::contract::ContractCounter::new("local:contract_counter", daemon.clone());

    let upload_res = contract_counter.upload();
    assert!(upload_res.is_ok());

    let init_res = contract_counter.instantiate(
        &InstantiateMsg { count: 0 },
        Some(&contract_counter.get_chain().sender()),
        None,
    );
    assert!(init_res.is_ok());

    let exec_res = contract_counter.execute(&ExecuteMsg::Increment {}, None);
    assert!(exec_res.is_ok());

    let query_res = contract_counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());
}
