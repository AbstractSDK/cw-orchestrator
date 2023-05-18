use cosmwasm_std::Addr;

use contract_counter::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use cw_orch::prelude::{CwOrcExecute, CwOrcInstantiate, CwOrcQuery, CwOrcUpload, Mock};

extern crate cw_orch;
mod counter_contract;

pub fn main() {
    let sender = Addr::unchecked("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y");

    let mock = Mock::new(&sender).unwrap();

    let contract_counter =
        contract_counter::contract::ContractCounter::new("mock:contract_counter", mock);

    let upload_res = contract_counter.upload();
    assert!(upload_res.is_ok());

    let init_res = contract_counter.instantiate(&InstantiateMsg { count: 0 }, Some(&sender), None);
    assert!(init_res.is_ok());

    let exec_res = contract_counter.execute(&ExecuteMsg::Increment {}, None);
    assert!(exec_res.is_ok());

    let query_res = contract_counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());
}
