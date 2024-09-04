use cosmwasm_std::instantiate2_address;
use cosmwasm_std::Api;
use cosmwasm_std::Binary;
use cw_orch_core::contract::interface_traits::ContractInstance;
use cw_orch_core::contract::interface_traits::CwOrchInstantiate;
use cw_orch_core::contract::interface_traits::CwOrchUpload;
use cw_orch_core::environment::DefaultQueriers;
use cw_orch_core::environment::TxHandler;
use cw_orch_core::environment::WasmQuerier;
use cw_orch_mock::MockBech32;
use mock_contract::InstantiateMsg;
use mock_contract::MockContract;

#[test]
fn instantiate2() -> anyhow::Result<()> {
    let app = MockBech32::new("mock");

    let salt = vec![12, 89, 156, 63];
    let mock_contract = MockContract::new("mock-contract", app.clone());

    mock_contract.upload()?;

    let expected_address = app.wasm_querier().instantiate2_addr(
        mock_contract.code_id()?,
        &app.sender_addr(),
        Binary::from(salt.clone()),
    )?;

    mock_contract.instantiate2(&InstantiateMsg {}, None, &[], Binary::new(salt.clone()))?;

    let addr = mock_contract.address()?;

    assert_eq!(addr.to_string(), expected_address);

    // Finally we need to make sure that the instantiate2 function also works inside contracts
    let canon_sender = app
        .app
        .borrow()
        .api()
        .addr_canonicalize(app.sender_addr().as_str())?;

    // If there is a `Invalid input: canonical address length not correct` error, that means this env doesn't work with instantiate2 correctly
    assert_eq!(
        addr.to_string(),
        app.app
            .borrow()
            .api()
            .addr_humanize(&instantiate2_address(
                app.app
                    .borrow()
                    .wrap()
                    .query_wasm_code_info(mock_contract.code_id()?)?
                    .checksum
                    .as_slice(),
                &canon_sender,
                &salt
            )?)?
            .to_string()
    );

    Ok(())
}
