mod tests {
    /*
        DaemonAsync contract general tests
    */

    use cw_orch_core::contract::interface_traits::*;
    use cw_orch_mock::Mock;
    use mock_contract::{InstantiateMsg, MigrateMsg, QueryMsg};

    use cosmwasm_std::Addr;

    use speculoos::prelude::*;

    #[test]
    fn helper_traits() {
        let sender = Addr::unchecked("sender");
        let chain = Mock::new(&sender);

        let contract = mock_contract::MockContract::new("test:mock_contract", chain.clone());

        asserting!("address is not present")
            .that(&contract.address())
            .is_err();

        asserting!("upload_if_needed is ok")
            .that(&contract.upload_if_needed())
            .is_ok();

        asserting!("latest_is_uploaded is true")
            .that(&contract.latest_is_uploaded().unwrap())
            .is_false(); // This is false, because of how checksum works in cw-multi-test

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

    // #[test]
    // #[serial_test::serial]
    // fn wrong_min_fee() {
    //     use cw_orch::prelude::networks;

    //     let runtime = tokio::runtime::Runtime::new().unwrap();

    //     let mut chain = networks::UNI_6;
    //     chain.gas_price = 0.00001;

    //     let daemon = Daemon::builder()
    //         .chain(chain)
    //         .handle(runtime.handle())
    //         .mnemonic("tide genuine angle mass fall promote blind skull swim army maximum add peasant fringe uncle october female crisp voyage blind extend jeans give wrap")
    //         .build()
    //         .unwrap();

    //     let contract = mock_contract::MockContract::new(
    //         format!("test:mock_contract:{}", Id::new()),
    //         daemon.clone(),
    //     );

    //     contract.upload().unwrap();
    // }

    #[test]
    fn cw_orch_interface_traits() {
        let sender = Addr::unchecked("sender");
        let chain = Mock::new(&sender);

        let contract = mock_contract::MockContract::new("test:mock_contract", chain.clone());

        // upload contract
        let upload_res = contract.upload();
        asserting!("upload is successful").that(&upload_res).is_ok();

        let code_id = contract.code_id().unwrap();

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
            code_id,
        );
        asserting!("migrate is successful")
            .that(&migrate_res)
            .is_ok();

        asserting!("that upload_if_needed returns None")
            .that(&contract.upload_if_needed().unwrap())
            .is_some(); // This is false, because of how checksum works in cw-multi-test
    }

    #[test]
    fn contract_snapshots() -> anyhow::Result<()> {
        use counter_contract::CounterExecuteMsgFns;
        let sender = Addr::unchecked("sender");
        let chain = Mock::new(&sender);

        let contract =
            counter_contract::CounterContract::new("test:counter_contract", chain.clone());
        contract.upload()?;
        contract.instantiate(
            &counter_contract::msg::InstantiateMsg { count: 0 },
            None,
            None,
        )?;

        contract.increment()?;

        chain.take_snapshot()?;

        Ok(())
    }
}
