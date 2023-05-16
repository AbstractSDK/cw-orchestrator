mod common;
#[cfg(feature = "node-tests")]
mod tests {
    /*
        DaemonAsync contract general tests
    */
    use crate::common;
    use cw_orch::prelude::*;
    use std::sync::Arc;

    use cosmwasm_std::Addr;

    use speculoos::prelude::*;
    use tokio::runtime::Runtime;

    #[test]
    #[serial_test::serial]
    fn helper_traits() {
        let runtime = Arc::new(Runtime::new().unwrap());

        let (sender, contract) = common::contract::start(&runtime);

        asserting!("address is not present")
            .that(&contract.address())
            .is_err();

        asserting!("upload_if_needed is ok")
            .that(&contract.upload_if_needed())
            .is_ok();

        asserting!("latest_is_uploaded is true")
            .that(&contract.latest_is_uploaded().unwrap())
            .is_true();

        let init_msg = common::contract::get_init_msg(&sender);

        let _ = contract.instantiate(&init_msg, Some(&Addr::unchecked(sender)), Some(&[]));

        asserting!("address is present")
            .that(&contract.address())
            .is_ok();

        asserting!("migrate_if_needed is none")
            .that(
                &contract
                    .migrate_if_needed(&cw20_base::msg::MigrateMsg {})
                    .unwrap(),
            )
            .is_none();

        asserting!("is_running_latest is true")
            .that(&contract.is_running_latest().unwrap())
            .is_true();

        let _ = contract.upload();

        asserting!("is_running_latest is false")
            .that(&contract.is_running_latest().unwrap())
            .is_false();

        asserting!("migrate_if_needed is some")
            .that(
                &contract
                    .migrate_if_needed(&cw20_base::msg::MigrateMsg {})
                    .unwrap(),
            )
            .is_some();

        asserting!("code_id is ok")
            .that(&contract.code_id())
            .is_ok();
    }

    #[test]
    #[serial_test::serial]
    fn cw_orch_x() {
        let runtime = Runtime::new().unwrap();

        let (sender, contract) = common::contract::start(&runtime);

        // upload contract
        let upload_res = contract.upload();
        asserting!("upload is successful").that(&upload_res).is_ok();

        let code_id = upload_res.unwrap().logs[0].events[1].attributes[1]
            .value
            .clone();

        log::info!("Using code_id {}", code_id);

        // init msg for contract
        let init_msg = common::contract::get_init_msg(&sender);

        // instantiate contract on chain
        let init_res = contract.instantiate(&init_msg, Some(&sender), None);
        asserting!("instantiate is successful")
            .that(&init_res)
            .is_ok();

        // do a query and validate its successful
        let query_res =
            contract.query::<cw20::BalanceResponse>(&cw20_base::msg::QueryMsg::Balance {
                address: sender.to_string(),
            });
        asserting!("query is successful").that(&query_res).is_ok();

        // validate migrations are successful
        let migrate_res = contract.migrate(
            &cw20_base::msg::MigrateMsg {},
            code_id.parse::<u64>().unwrap(),
        );
        asserting!("migrate is successful")
            .that(&migrate_res)
            .is_ok();

        asserting!("that upload_if_needed returns None")
            .that(&contract.upload_if_needed().unwrap())
            .is_none();
    }
}
