use cw_orch::remote_mock::RemoteMock;
use cw_orch::daemon::networks::JUNO_1;


use counter_contract::{
    contract::CounterContract,
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
};
use cw_orch::prelude::{CwOrchExecute, CwOrchInstantiate, CwOrchQuery, CwOrchUpload};
use tokio::runtime::Runtime;

pub fn main() {

    env_logger::init();


    let rt = Runtime::new().unwrap();

    let mock = RemoteMock::new(JUNO_1, rt.handle());

    let contract_counter = CounterContract::new("mock:contract_counter", mock.clone());

    let upload_res = contract_counter.upload();
    assert!(upload_res.is_ok());

    let init_res = contract_counter.instantiate(&InstantiateMsg { count: 0 }, Some(&mock.sender), None);
    assert!(init_res.is_ok());

    let exec_res = contract_counter.execute(&ExecuteMsg::Increment {}, None);
    assert!(exec_res.is_ok());

    let query_res = contract_counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());

    let analysis = mock.analysis();
    analysis.compare_all_readable_contract_storage();
}
