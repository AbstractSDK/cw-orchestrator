use cosmwasm_std::Addr;

use counter_contract::{
    contract::CounterContract,
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
};
use cw_orch::prelude::{CwOrcExecute, CwOrcInstantiate, CwOrcQuery, CwOrcUpload, Mock};

pub fn main() {
    let sender = Addr::unchecked("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y");

    let mock = Mock::new(&sender);

    let contract_counter = CounterContract::new("mock:contract_counter", mock);

    let upload_res = contract_counter.upload();
    assert!(upload_res.is_ok());

    let init_res = contract_counter.instantiate(&InstantiateMsg { count: 0 }, Some(&sender), None);
    assert!(init_res.is_ok());

    let exec_res = contract_counter.execute(&ExecuteMsg::Increment {}, None);
    assert!(exec_res.is_ok());

    let query_res = contract_counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());
}
