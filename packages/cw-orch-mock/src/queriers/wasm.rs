use std::{cell::RefCell, rc::Rc};

use cosmwasm_std::{to_json_binary, ContractInfoResponse, Empty};
use cw_multi_test::BasicApp;
use cw_orch_core::{
    contract::interface_traits::{ContractInstance, Uploadable},
    environment::{Querier, QuerierGetter, QueryHandler, StateInterface, TxHandler, WasmQuerier},
    CwEnvError,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::Mock;

pub struct MockWasmQuerier {
    app: Rc<RefCell<BasicApp<Empty, Empty>>>,
}

impl MockWasmQuerier {
    fn new<S: StateInterface>(mock: &Mock<S>) -> Self {
        Self {
            app: mock.app.clone(),
        }
    }
}

impl Querier for MockWasmQuerier {
    type Error = CwEnvError;
}

impl<S: StateInterface> QuerierGetter<MockWasmQuerier> for Mock<S> {
    fn querier(&self) -> MockWasmQuerier {
        MockWasmQuerier::new(self)
    }
}

impl WasmQuerier for MockWasmQuerier {
    fn code_id_hash(&self, code_id: u64) -> Result<String, CwEnvError> {
        let code_info = self.app.borrow().wrap().query_wasm_code_info(code_id)?;
        Ok(code_info.checksum.to_string())
    }

    /// Returns the code_info structure of the provided contract
    fn contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<ContractInfoResponse, CwEnvError> {
        let info = self.app.borrow().wrap().query_wasm_contract_info(address)?;
        Ok(info)
    }

    fn local_hash<Chain: TxHandler + QueryHandler, T: Uploadable + ContractInstance<Chain>>(
        &self,
        contract: &T,
    ) -> Result<String, CwEnvError> {
        // We return the hashed contract-id.
        // This will cause the logic to never re-upload a contract if it has the same contract-id.
        Ok(sha256::digest(contract.id().as_bytes()))
    }

    fn raw_query(
        &self,
        address: impl Into<String>,
        query_data: Vec<u8>,
    ) -> Result<Vec<u8>, Self::Error> {
        Ok(self
            .app
            .borrow()
            .wrap()
            .query(&cosmwasm_std::QueryRequest::Wasm(
                cosmwasm_std::WasmQuery::Raw {
                    contract_addr: address.into(),
                    key: query_data.into(),
                },
            ))?)
    }

    fn smart_query<Q, T>(
        &self,
        address: impl Into<String>,
        query_data: &Q,
    ) -> Result<T, Self::Error>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        Ok(self
            .app
            .borrow()
            .wrap()
            .query(&cosmwasm_std::QueryRequest::Wasm(
                cosmwasm_std::WasmQuery::Smart {
                    contract_addr: address.into(),
                    msg: to_json_binary(query_data)?,
                },
            ))?)
    }

    fn code(&self, code_id: u64) -> Result<cosmwasm_std::CodeInfoResponse, Self::Error> {
        Ok(self
            .app
            .borrow()
            .wrap()
            .query(&cosmwasm_std::QueryRequest::Wasm(
                cosmwasm_std::WasmQuery::CodeInfo { code_id },
            ))?)
    }
}
