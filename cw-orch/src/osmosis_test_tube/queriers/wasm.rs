use std::{cell::RefCell, rc::Rc};

use cosmwasm_std::{from_json, to_json_vec, CodeInfoResponse, ContractInfoResponse};
use cw_orch_core::{
    environment::{Querier, QuerierGetter, StateInterface, WasmQuerier},
    CwEnvError,
};
use osmosis_test_tube::{OsmosisTestApp, Runner};

use crate::osmosis_test_tube::{map_err, OsmosisTestTube};
use osmosis_std::types::cosmwasm::wasm::v1::{
    QueryCodeRequest, QueryCodeResponse, QueryContractInfoRequest, QueryContractInfoResponse,
    QueryRawContractStateRequest, QueryRawContractStateResponse, QuerySmartContractStateRequest,
    QuerySmartContractStateResponse,
};

pub struct OsmosisTestTubeWasmQuerier {
    app: Rc<RefCell<OsmosisTestApp>>,
}

impl OsmosisTestTubeWasmQuerier {
    fn new<S: StateInterface>(mock: &OsmosisTestTube<S>) -> Self {
        Self {
            app: mock.app.clone(),
        }
    }
}

impl Querier for OsmosisTestTubeWasmQuerier {
    type Error = CwEnvError;
}

impl<S: StateInterface> QuerierGetter<OsmosisTestTubeWasmQuerier> for OsmosisTestTube<S> {
    fn querier(&self) -> OsmosisTestTubeWasmQuerier {
        OsmosisTestTubeWasmQuerier::new(self)
    }
}

impl WasmQuerier for OsmosisTestTubeWasmQuerier {
    fn code_id_hash(&self, code_id: u64) -> Result<String, Self::Error> {
        let code_info_result: QueryCodeResponse = self
            .app
            .borrow()
            .query(
                "/cosmwasm.wasm.v1.Query/Code",
                &QueryCodeRequest { code_id },
            )
            .map_err(map_err)?;

        Ok(hex::encode(
            code_info_result
                .code_info
                .ok_or(CwEnvError::CodeIdNotInStore(code_id.to_string()))?
                .data_hash,
        ))
    }

    fn contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<ContractInfoResponse, CwEnvError> {
        let address = address.into();
        let result = self
            .app
            .borrow()
            .query::<_, QueryContractInfoResponse>(
                "/cosmwasm.wasm.v1.Query/ContractInfo",
                &QueryContractInfoRequest {
                    address: address.clone(),
                },
            )
            .map_err(map_err)?
            .contract_info
            .ok_or(CwEnvError::AddrNotInStore(address))?;

        let mut contract_info = ContractInfoResponse::default();
        contract_info.code_id = result.code_id;
        contract_info.creator = result.creator;
        contract_info.admin = Some(result.admin);

        contract_info.ibc_port = if result.ibc_port_id.is_empty() {
            None
        } else {
            Some(result.ibc_port_id)
        };

        Ok(contract_info)
    }

    fn raw_query(
        &self,
        address: impl Into<String>,
        query_data: Vec<u8>,
    ) -> Result<Vec<u8>, Self::Error> {
        let address = address.into();
        let result = self
            .app
            .borrow()
            .query::<_, QueryRawContractStateResponse>(
                "/cosmwasm.wasm.v1.Query/RawContractState",
                &QueryRawContractStateRequest {
                    address: address.clone(),
                    query_data,
                },
            )
            .map_err(map_err)?
            .data;

        Ok(result)
    }

    fn smart_query<Q: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        address: impl Into<String>,
        query_data: &Q,
    ) -> Result<T, Self::Error> {
        let address = address.into();
        let result = self
            .app
            .borrow()
            .query::<_, QuerySmartContractStateResponse>(
                "/cosmwasm.wasm.v1.Query/RawContractState",
                &QuerySmartContractStateRequest {
                    address: address.clone(),
                    query_data: to_json_vec(query_data)?,
                },
            )
            .map_err(map_err)?
            .data;

        Ok(from_json(result)?)
    }

    fn code(&self, code_id: u64) -> Result<cosmwasm_std::CodeInfoResponse, Self::Error> {
        let response: QueryCodeResponse = self
            .app
            .borrow()
            .query(
                "/cosmwasm.wasm.v1.Query/Code",
                &QueryCodeRequest { code_id },
            )
            .map_err(map_err)?;

        let code_info = response
            .code_info
            .ok_or(CwEnvError::CodeIdNotInStore(code_id.to_string()))?;

        let mut c = CodeInfoResponse::default();
        c.code_id = code_id;
        c.creator = code_info.creator;
        c.checksum = code_info.data_hash.into();

        Ok(c)
    }
}
