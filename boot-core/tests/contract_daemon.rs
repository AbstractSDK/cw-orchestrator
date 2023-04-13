/*
Contract<Daemon> tests
run with:
RUST_LOG=info cargo test --package boot-core:0.10.0 --test contract_daemon -- contract_daemon::general --exact --nocapture
*/
mod common;
#[cfg(test)]
mod contract_daemon {
    use std::sync::Arc;

    use cosmwasm_std::{Addr, Uint128};
    // use cosmwasm_std::Addr;
    // use cw_multi_test::ContractWrapper;
    use tokio::runtime::Runtime;

    use speculoos::prelude::*;

    use boot_core::{
        instantiate_daemon_env, Contract, ContractWrapper, DaemonOptionsBuilder, TxHandler,
    };

    const CW20_CONTRACT_WASM: &str = "/../boot-cw-plus/cw-artifacts/cw20_base.wasm";

    #[test]
    fn general() {
        let runtime = Arc::new(Runtime::new().unwrap());

        let options = DaemonOptionsBuilder::default()
            .network(boot_core::networks::LOCAL_JUNO)
            .deployment_id("v0.1.0")
            .build()
            .unwrap();

        let (sender, chain) = instantiate_daemon_env(&runtime, options).unwrap();

        let info = chain.block_info().unwrap();
        asserting!("block height is 1")
            .that(&info.height)
            .is_equal_to(&1);

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

        // asserting!("address is not present")
        //     .that(&contract.address())
        //     .is_err();

        asserting!("upload_if_needed is ok")
            .that(&contract.upload_if_needed())
            .is_ok();

        asserting!("latest_is_uploaded is true")
            .that(&contract.latest_is_uploaded().unwrap())
            .is_true();

        // asserting!("address is not present")
        //     .that(&contract.address())
        //     .is_err();

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

        let _ = contract.instantiate(&init_msg, Some(&Addr::unchecked(sender)), Some(&vec![]));

        // asserting!("address is present")
        //     .that(&contract.address())
        //     .is_ok();

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

        // TODO: assert get_wasm_code_path
        // TODO: assert checksum
    }
}
