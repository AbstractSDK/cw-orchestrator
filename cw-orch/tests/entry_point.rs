use cw_orch::prelude::CwOrcExecute;
use common::mock_contract::QueryMsg;
use cw_orch::prelude::ContractInstance;
use cw_orch::prelude::CwOrcMigrate;
use cw_orch::prelude::CwOrcQuery;
use cosmwasm_std::Event;

use cw_orch::prelude::CwOrcUpload;
mod common;
use cosmwasm_std::Addr;
use common::mock_contract::{self, InstantiateMsg, ExecuteMsg};
use cw_orch::prelude::{Mock, CwOrcInstantiate};

use crate::common::mock_contract::MigrateMsg;

#[test]
fn test_instantiate(){
	let contract = mock_contract::CwOrch::new("test:mock_contract", Mock::new(&Addr::unchecked("Ghazshag")).unwrap());
	contract.upload().unwrap();

	contract.instantiate(&InstantiateMsg{
	},None, None).unwrap();
}

#[test]
fn test_execute(){

	let contract = mock_contract::CwOrch::new("test:mock_contract", Mock::new(&Addr::unchecked("Ghazshag")).unwrap());
	contract.upload().unwrap();

	contract.instantiate(&InstantiateMsg{
	},None, None).unwrap();

	let response = contract.execute(&ExecuteMsg::FirstMessage {  }, None).unwrap();
	response.has_event(&Event::new("wasm").add_attribute("_contract_addr", "contract0").add_attribute("action", "first message passed"));

	contract.execute(&ExecuteMsg::SecondMessage { t: "".to_string() }, None).unwrap_err();
}

#[test]
fn test_query(){

	let contract = mock_contract::CwOrch::new("test:mock_contract", Mock::new(&Addr::unchecked("Ghazshag")).unwrap());
	contract.upload().unwrap();

	contract.instantiate(&InstantiateMsg{
	},None, None).unwrap();

	let response: String = contract.query(&QueryMsg::FirstQuery {  }).unwrap();
	assert_eq!(response, "first query passed");

	contract.query::<String>(&QueryMsg::SecondQuery {  }).unwrap_err();
}

#[test]
fn test_migrate(){
	let admin = Addr::unchecked("Ghazshag");
	let contract = mock_contract::CwOrch::new("test:mock_contract", Mock::new(&admin).unwrap());
	contract.upload().unwrap();

	contract.instantiate(&InstantiateMsg{
	},Some(&admin), None).unwrap();

	contract.migrate(&MigrateMsg {t: "error".to_string()  }, contract.code_id().unwrap()).unwrap_err();
	let response = contract.migrate(&MigrateMsg {t: "success".to_string()  }, contract.code_id().unwrap()).unwrap();
	assert_eq!(response.events.len(), 1);
}