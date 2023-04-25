/*
    Daemon tests
*/
mod common;
use std::sync::Arc;

use cw_orchestrate::*;
use speculoos::prelude::*;

use cw20_base::msg::*;
use tokio::runtime::Runtime;

#[contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20Base;

#[test]
fn general() {
    let runtime = Arc::new(Runtime::new().unwrap());

    let (sender, contract) = common::contract::start(&runtime);

    // upload contract
    let upload_res = contract.upload();
    asserting!("upload is succesful").that(&upload_res).is_ok();

    let code_id = upload_res.unwrap().logs[0].events[1].attributes[1]
        .value
        .clone();

    log::info!("Using code_id {}", code_id);

    // init msg for contract
    let init_msg = common::contract::get_init_msg(&sender);

    // instantiate contract on chain
    let init_res = contract.instantiate(&init_msg, Some(&sender.clone()), None);
    asserting!("instantiate is successful")
        .that(&init_res)
        .is_ok();

    // do a query and validate its successful
    let query_res = contract.query::<cw20::BalanceResponse>(&cw20_base::msg::QueryMsg::Balance {
        address: sender.to_string(),
    });
    asserting!("query is successful").that(&query_res).is_ok();

    // validate migrations are successful
    let migrate_res = contract.migrate(&MigrateMsg {}, code_id.parse::<u64>().unwrap());
    asserting!("migrate is successful")
        .that(&migrate_res)
        .is_ok();

    asserting!("that upload_if_needed returns None")
        .that(&contract.upload_if_needed().unwrap())
        .is_none();
}
