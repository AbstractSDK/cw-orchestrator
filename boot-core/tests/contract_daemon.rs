/*
Contract<Daemon> tests
run with:
RUST_LOG=info cargo test --package boot-core:0.10.0 --test contract_daemon -- contract_daemon::general --exact --nocapture
*/
mod common;
#[cfg(test)]
mod contract_daemon {
    use std::sync::Arc;

    // use cosmwasm_std::Addr;
    // use cw_multi_test::ContractWrapper;
    use tokio::runtime::Runtime;

    use speculoos::prelude::*;

    use boot_core::{
        instantiate_daemon_env, Contract, ContractWrapper, DaemonOptionsBuilder, TxHandler,
    };

    const CW20_CONTRACT_WASM: &str = "/../boot-cw-plus/cw-artifacts/cw20_base.wasm";
    // const SENDER: &str = "cosmos123";

    #[test]
    fn general() {
        let runtime = Arc::new(Runtime::new().unwrap());

        let options = DaemonOptionsBuilder::default()
            .network(boot_core::networks::LOCAL_JUNO)
            .deployment_id("v0.1.0")
            .build()
            .unwrap();

        let (_, chain) = instantiate_daemon_env(&runtime, options).unwrap();

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

        let code_id = contract.code_id();
        println!("code_id: {:#?}", code_id);

        let upload_if_needed_res = contract.upload_if_needed();
        println!("upload_if_needed_res: {:#?}", upload_if_needed_res);

        let _res = contract.upload();

        let code_id = contract.code_id();
        println!("code_id: {:#?}", code_id);

        let latest_is_uploaded_res = contract.latest_is_uploaded();
        println!("latest_is_uploaded_res: {:#?}", latest_is_uploaded_res);
    }
}
