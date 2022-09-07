use cosmwasm_std::{Addr, Event};
use cw_multi_test::{AppResponse, App, Executor};
use serde::{Serialize, de::DeserializeOwned};

use crate::{tx_handler::TxHandler, contract::ContractCodeReference, CosmScriptError};
use std::{fmt::Debug, cell::RefCell};
struct MockChain {
    sender: Addr,
    app: RefCell<App>,
}

// Execute on the test chain, returns test response type
impl TxHandler for MockChain {
    type Response = AppResponse;

    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[cosmwasm_std::Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, crate::CosmScriptError> {
        self.app.borrow_mut().execute_contract(self.sender.clone(), contract_address.to_owned(),exec_msg, coins).map_err(From::from)
    }

    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: &str,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<Self::Response, crate::CosmScriptError> {
        let addr = self.app.borrow_mut().instantiate_contract(code_id, self.sender.clone(), init_msg,coins, label, admin.map(|a| a.to_string()))?;
        // add contract address to events manually 
        let mut event = Event::new("instantiate");
        event.add_attribute("_contract_address", addr);
        let mut resp = AppResponse::default();
        resp.events = vec![event];
        Ok(resp)
    }

    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, crate::CosmScriptError> {
        self.app.borrow().wrap().query_wasm_smart(contract_address, query_msg).map_err(From::from)
    }

    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, crate::CosmScriptError> {
        self.app.borrow_mut().migrate_contract(self.sender, contract_address.clone(), migrate_msg, new_code_id).map_err(From::from)
    }

    fn upload(&self, contract_source: ContractCodeReference) -> Result<Self::Response, crate::CosmScriptError> {
        if let ContractCodeReference::ContractEndpoints(contract) = contract_source{
            let code_id = self.app.borrow_mut().store_code(contract);
            // add contract code_id to events manually 
            let mut event = Event::new("store_code");
            event.add_attribute("code_id", code_id.to_string());
            let mut resp = AppResponse::default();
            resp.events = vec![event];
            Ok(resp)
        } else {
            Err(CosmScriptError::StdErr("Contract reference must be cosm-multi-test contract object.".into()))
        }
    }

    
    
}