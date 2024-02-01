use cosmwasm_std::to_json_binary;
use cosmwasm_std::Binary;
use cosmwasm_std::HexBinary;
use cw_multi_test::Executor;
use cw_orch_core::contract::interface_traits::ContractInstance;
use cw_orch_core::contract::interface_traits::CwOrchUpload;
use cw_orch_core::environment::TxHandler;
use cw_orch_mock::Mock;
use cw_utils::parse_instantiate_response_data;
use mock_contract::InstantiateMsg;
use mock_contract::MockContract;

#[test]
fn instantiate2() -> anyhow::Result<()> {
    let app = Mock::new("sender");

    let mock_contract = MockContract::new("mock-contract", app.clone());

    mock_contract.upload()?;

    let salt = Binary(vec![12, 89, 156, 63]);

    let execution_response = app.app.borrow_mut().execute(
        app.sender(),
        cosmwasm_std::CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Instantiate2 {
            admin: None,
            code_id: mock_contract.code_id()?,
            label: "Weird label".to_string(),
            msg: to_json_binary(&InstantiateMsg {})?,
            funds: vec![],
            salt: salt.clone(),
        }),
    )?;

    let addr = parse_instantiate_response_data(&execution_response.data.unwrap())?;

    assert_eq!(
        addr.contract_address,
        format!("contract/sender/{}", HexBinary::from(salt).to_hex())
    );

    Ok(())
}
