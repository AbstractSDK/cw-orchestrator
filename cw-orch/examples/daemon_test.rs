use std::env;

use contract_counter::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use cw_orch::prelude::{
    networks, ContractInstance, CwOrcExecute, CwOrcInstantiate, CwOrcQuery, CwOrcUpload, Daemon,
    TxHandler,
};
use tokio::runtime::Runtime;

extern crate cw_orch;
mod counter_contract;

pub fn main() {
    let runtime = Runtime::new().unwrap();

    let res = Daemon::builder()
        .chain(networks::parse_network(&env::var("CHAIN").unwrap()))
        .handle(runtime.handle())
        .mnemonic(env::var("LOCAL_MNEMONIC").unwrap())
        .build();

    let Some(daemon) = res.as_ref().ok() else {
        panic!("Error: {}", res.err().unwrap());
    };

    let contract_counter =
        contract_counter::contract::ContractCounter::new("local:contract_counter", daemon.clone());

    let upload_res = contract_counter.upload().unwrap();
    println!("upload_res: {:#?}", upload_res);

    let init_res = contract_counter
        .instantiate(
            &InstantiateMsg { count: 0 },
            Some(&contract_counter.get_chain().sender()),
            None,
        )
        .unwrap();
    println!("init_res: {:#?}", init_res);

    let exec_res = contract_counter
        .execute(&ExecuteMsg::Increment {}, None)
        .unwrap();
    println!("exec_res: {:#?}", exec_res);

    let query_res = contract_counter
        .query::<GetCountResponse>(&QueryMsg::GetCount {})
        .unwrap();

    println!("query_res: {:#?}", query_res);
}
