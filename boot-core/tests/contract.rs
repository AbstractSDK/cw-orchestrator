/*

*/
mod common;
#[cfg(test)]
mod contract {
    use std::{env, sync::Arc};

    use tokio::runtime::Runtime;

    use cosmwasm_std::Uint128;
    use cw_multi_test::ContractWrapper;

    use speculoos::prelude::*;

    use boot_core::{
        contract, instantiate_daemon_env, networks::LOCAL_JUNO, Contract, DaemonOptionsBuilder,
    };

    use cw20_base::msg::*;

    const CW20_CONTRACT_WASM: &str = "/../boot-cw-plus/cw-artifacts/cw20_base.wasm";

    #[contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
    pub struct Cw20Base;

    #[test]
    fn general() {
        let rt = Arc::new(Runtime::new().unwrap());

        // configure daemon options
        let options = DaemonOptionsBuilder::default()
            .network(LOCAL_JUNO)
            .deployment_id("v0.1.0")
            .build()
            .unwrap();

        // instantiate chain daemon
        let (sender, chain) = instantiate_daemon_env(&rt, options).unwrap();
        log::info!("Using wallet {}", sender);

        // create contract base configuration
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}{}", crate_path, CW20_CONTRACT_WASM);
        log::info!("Using wasm path {}", wasm_path);

        let mut contract = Contract::new("cw-plus:cw20_base", chain)
            .with_mock(Box::new(
                ContractWrapper::new_with_empty(
                    cw20_base::contract::execute,
                    cw20_base::contract::instantiate,
                    cw20_base::contract::query,
                )
                .with_migrate(cw20_base::contract::migrate),
            ))
            .with_wasm_path(wasm_path);

        // upload contract
        let upload_res = contract.upload();
        asserting!("upload is succesful").that(&upload_res).is_ok();

        let code_id = upload_res.unwrap().logs[0].events[1].attributes[1]
            .value
            .clone();

        log::info!("Using code_id {}", code_id);

        // init msg for contract
        let init_msg = cw20_base::msg::InstantiateMsg {
            name: "Token".to_owned(),
            symbol: "TOK".to_owned(),
            decimals: 6u8,
            initial_balances: vec![cw20::Cw20Coin {
                address: sender.to_string(),
                amount: Uint128::from(10000u128),
            }],
            mint: None,
            marketing: None,
        };

        // instantiate contract on chain
        let init_res = contract.instantiate(&init_msg, Some(&sender.clone()), None);
        asserting!("instantiate is successful")
            .that(&init_res)
            .is_ok();

        // do a query and validate its successful
        let query_res = contract.query::<cw20_base::msg::QueryMsg, cw20::BalanceResponse>(
            &cw20_base::msg::QueryMsg::Balance {
                address: sender.to_string(),
            },
        );
        asserting!("query is successful").that(&query_res).is_ok();

        // validate migrations are successful
        let migrate_res = contract.migrate(&MigrateMsg {}, code_id.parse::<u64>().unwrap());
        asserting!("migrate is successful")
            .that(&migrate_res)
            .is_ok();
    }
}
