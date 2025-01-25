use std::{cell::RefCell, marker::PhantomData, rc::Rc, str::FromStr};

use cosmwasm_std::{
    from_json, instantiate2_address, to_json_vec, Addr, CanonicalAddr, Checksum, CodeInfoResponse,
    ContractInfoResponse,
};
use cw_orch_core::{
    contract::interface_traits::{ContractInstance, Uploadable},
    environment::{Querier, QuerierGetter, StateInterface, WasmQuerier},
    CwEnvError,
};
use osmosis_test_tube::cosmrs::AccountId;
use osmosis_test_tube::{OsmosisTestApp, Runner};

use crate::{map_err, OsmosisTestTube, MOCK_CHAIN_INFO};
use osmosis_test_tube::osmosis_std::types::cosmwasm::wasm::v1::{
    QueryCodeRequest, QueryCodeResponse, QueryContractInfoRequest, QueryContractInfoResponse,
    QueryRawContractStateRequest, QueryRawContractStateResponse, QuerySmartContractStateRequest,
    QuerySmartContractStateResponse,
};

pub struct OsmosisTestTubeWasmQuerier<S> {
    app: Rc<RefCell<OsmosisTestApp>>,
    _state: PhantomData<S>,
}

impl<S: StateInterface> OsmosisTestTubeWasmQuerier<S> {
    fn new(mock: &OsmosisTestTube<S>) -> Self {
        Self {
            app: mock.app.clone(),
            _state: PhantomData,
        }
    }
}

impl<S> Querier for OsmosisTestTubeWasmQuerier<S> {
    type Error = CwEnvError;
}

impl<S: StateInterface> QuerierGetter<OsmosisTestTubeWasmQuerier<S>> for OsmosisTestTube<S> {
    fn querier(&self) -> OsmosisTestTubeWasmQuerier<S> {
        OsmosisTestTubeWasmQuerier::new(self)
    }
}

impl<S: StateInterface> WasmQuerier for OsmosisTestTubeWasmQuerier<S> {
    type Chain = OsmosisTestTube<S>;
    fn code_id_hash(&self, code_id: u64) -> Result<Checksum, Self::Error> {
        let code_info_result: QueryCodeResponse = self
            .app
            .borrow()
            .query(
                "/cosmwasm.wasm.v1.Query/Code",
                &QueryCodeRequest { code_id },
            )
            .map_err(map_err)?;

        code_info_result
            .code_info
            .ok_or(CwEnvError::CodeIdNotInStore(code_id.to_string()))?
            .data_hash
            .as_slice()
            .try_into()
            .map_err(|e: cosmwasm_std::ChecksumError| CwEnvError::StdErr(e.to_string()))
    }

    fn contract_info(&self, address: &Addr) -> Result<ContractInfoResponse, CwEnvError> {
        let address: String = address.to_string();
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

        let ibc_port = if result.ibc_port_id.is_empty() {
            None
        } else {
            Some(result.ibc_port_id)
        };
        let contract_info = ContractInfoResponse::new(
            result.code_id,
            Addr::unchecked(result.creator),
            Some(Addr::unchecked(result.admin)),
            false,
            ibc_port,
        );
        Ok(contract_info)
    }

    fn raw_query(&self, address: &Addr, query_data: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        let address = address.to_string();
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
        address: &Addr,
        query_data: &Q,
    ) -> Result<T, Self::Error> {
        let address = address.to_string();
        let result = self
            .app
            .borrow()
            .query::<_, QuerySmartContractStateResponse>(
                "/cosmwasm.wasm.v1.Query/SmartContractState",
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

        let c = CodeInfoResponse::new(
            code_id,
            Addr::unchecked(code_info.creator),
            code_info
                .data_hash
                .as_slice()
                .try_into()
                .map_err(|e: cosmwasm_std::ChecksumError| CwEnvError::StdErr(e.to_string()))?,
        );
        Ok(c)
    }

    fn instantiate2_addr(
        &self,
        code_id: u64,
        creator: &Addr,
        salt: cosmwasm_std::Binary,
    ) -> Result<String, Self::Error> {
        let checksum = self.code_id_hash(code_id)?;

        let creator_str = creator.to_string();
        let account_id = AccountId::from_str(&creator_str).unwrap();
        let prefix = account_id.prefix();
        let canon = account_id.to_bytes();
        let addr =
            instantiate2_address(checksum.as_slice(), &CanonicalAddr::from(canon), &salt).unwrap();

        Ok(AccountId::new(prefix, addr.as_slice()).unwrap().to_string())
    }

    fn local_hash<T: Uploadable + ContractInstance<Self::Chain>>(
        &self,
        _contract: &T,
    ) -> Result<Checksum, CwEnvError> {
        <T as Uploadable>::wasm(&MOCK_CHAIN_INFO.into()).checksum()
    }
}
