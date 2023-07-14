use cosmwasm_std::Addr;

use counter_contract::{
    contract::CounterContract,
    msg::{ExecuteMsg, ExecuteMsgFns, GetCountResponse, InstantiateMsg, QueryMsg, QueryMsgFns},
};
use cw_orch::prelude::{CwOrchExecute, CwOrchInstantiate, CwOrchQuery, CwOrchUpload, Mock};

pub fn main() {
    let sender = Addr::unchecked("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y");

    let mock = Mock::new(&sender);

    let contract_counter = CounterContract::new("mock:contract_counter", mock);

    let upload_res = contract_counter.upload();
    upload_res.unwrap();

    let init_res = contract_counter.instantiate(&InstantiateMsg { count: 0 }, Some(&sender), None);
    init_res.unwrap();

    let exec_res = contract_counter.execute(&ExecuteMsg::Increment {}, None);
    exec_res.unwrap();

    let query_res = contract_counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert_eq!(query_res.unwrap().count, 1);
}
