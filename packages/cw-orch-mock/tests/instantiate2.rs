use cosmwasm_std::Binary;
use cosmwasm_std::HexBinary;
use cw_orch_core::contract::interface_traits::ContractInstance;
use cw_orch_core::contract::interface_traits::CwOrchInstantiate;
use cw_orch_core::contract::interface_traits::CwOrchUpload;
use cw_orch_mock::Mock;
use mock_contract::InstantiateMsg;
use mock_contract::MockContract;

#[test]
fn instantiate2() -> anyhow::Result<()> {
    let app = Mock::new("sender");

    let salt = Binary(vec![12, 89, 156, 63]);
    let mock_contract = MockContract::new("mock-contract", app.clone());

    mock_contract.upload()?;

    mock_contract.instantiate2(&InstantiateMsg {}, None, None, salt.clone())?;

    let addr = mock_contract.address()?;

    assert_eq!(
        addr.to_string(),
        format!("contract/sender/{}", HexBinary::from(salt).to_hex())
    );

    Ok(())
}
