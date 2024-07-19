mod common;
#[cfg(feature = "node-tests")]
mod tests {
    /*
        DaemonAsync contract general tests
    */

    use cw_orch_core::{contract::interface_traits::*, environment::TxHandler};
    use cw_orch_daemon::Daemon;
    use mock_contract::{InstantiateMsg, MigrateMsg, QueryMsg};

    use cosmwasm_std::Addr;

    use speculoos::prelude::*;

    #[test]
    #[serial_test::serial]
    fn helper_traits() {
        use cw_orch_networks::networks;

        let mut daemon = Daemon::builder(networks::LOCAL_JUNO)
            .is_test(true)
            .build()
            .unwrap();

        daemon.flush_state().unwrap();

        let sender = daemon.sender_addr();

        let contract = mock_contract::MockContract::new("test:mock_contract", daemon.clone());

        asserting!("address is not present")
            .that(&contract.address())
            .is_err();

        asserting!("upload_if_needed is ok")
            .that(&contract.upload_if_needed())
            .is_ok();

        asserting!("latest_is_uploaded is true")
            .that(&contract.latest_is_uploaded().unwrap())
            .is_true();

        let init_msg = &InstantiateMsg {};

        let _ = contract.instantiate(init_msg, Some(&Addr::unchecked(sender)), Some(&[]));

        asserting!("address is present")
            .that(&contract.address())
            .is_ok();

        asserting!("migrate_if_needed is none")
            .that(
                &contract
                    .migrate_if_needed(&MigrateMsg {
                        t: "success".to_string(),
                    })
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
                    .migrate_if_needed(&MigrateMsg {
                        t: "success".to_string(),
                    })
                    .unwrap(),
            )
            .is_some();

        asserting!("code_id is ok")
            .that(&contract.code_id())
            .is_ok();
    }

    #[test]
    #[serial_test::serial]
    fn cw_orch_interface_traits() {
        use cw_orch_networks::networks;

        let daemon = Daemon::builder(networks::LOCAL_JUNO)
            .is_test(true)
            .build()
            .unwrap();

        let sender = daemon.sender_addr();

        let contract = mock_contract::MockContract::new("test:mock_contract", daemon.clone());

        // upload contract
        let upload_res = contract.upload();
        asserting!("upload is successful").that(&upload_res).is_ok();

        let code_id = upload_res.unwrap().logs[0].events[1].attributes[1]
            .value
            .clone();

        log::info!("Using code_id {}", code_id);

        // instantiate contract on chain
        let init_res = contract.instantiate(&InstantiateMsg {}, Some(&sender), None);
        asserting!("instantiate is successful")
            .that(&init_res)
            .is_ok();

        // do a query and validate its successful
        let query_res = contract.query::<String>(&QueryMsg::FirstQuery {});
        asserting!("query is successful").that(&query_res).is_ok();

        // validate migrations are successful
        let migrate_res = contract.migrate(
            &MigrateMsg {
                t: "success".to_string(),
            },
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
