use crate::{map_err, NeutronTestTube, MOCK_CHAIN_INFO};

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
use neutron_test_tube::{
    cosmrs::AccountId,
    neutron_std::types::cosmwasm::wasm::v1::{
        QueryCodeRequest, QueryCodeResponse, QueryContractInfoRequest, QueryContractInfoResponse,
        QueryRawContractStateRequest, QueryRawContractStateResponse,
        QuerySmartContractStateRequest, QuerySmartContractStateResponse,
    },
    NeutronTestApp, Runner,
};

pub struct NeutronTestTubeWasmQuerier<S> {
    app: Rc<RefCell<NeutronTestApp>>,
    _state: PhantomData<S>,
}

impl<S: StateInterface> NeutronTestTubeWasmQuerier<S> {
    fn new(mock: &NeutronTestTube<S>) -> Self {
        Self {
            app: mock.app.clone(),
            _state: PhantomData,
        }
    }
}

impl<S> Querier for NeutronTestTubeWasmQuerier<S> {
    type Error = CwEnvError;
}

impl<S: StateInterface> QuerierGetter<NeutronTestTubeWasmQuerier<S>> for NeutronTestTube<S> {
    fn querier(&self) -> NeutronTestTubeWasmQuerier<S> {
        NeutronTestTubeWasmQuerier::new(self)
    }
}

impl<S: StateInterface> WasmQuerier for NeutronTestTubeWasmQuerier<S> {
    type Chain = NeutronTestTube<S>;
    fn code_id_hash(&self, code_id: u64) -> Result<Checksum, Self::Error> {
        let code_info_result: QueryCodeResponse = self
            .app
            .borrow()
            .query(
                "/cosmwasm.wasm.v1.Query/Code",
                &QueryCodeRequest { code_id },
            )
            .map_err(map_err)?;

        Checksum::try_from(
            code_info_result
                .code_info
                .ok_or(CwEnvError::CodeIdNotInStore(code_id.to_string()))?
                .data_hash
                .as_slice(),
        )
        .map_err(|checksum_error| CwEnvError::StdErr(checksum_error.to_string()))
    }

    fn contract_info(&self, address: &Addr) -> Result<ContractInfoResponse, CwEnvError> {
        let address = address.to_string();
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

        let ibc_port_id = if result.ibc_port_id.is_empty() {
            None
        } else {
            Some(result.ibc_port_id)
        };
        let admin = if result.admin.is_empty() {
            None
        } else {
            Some(Addr::unchecked(result.admin))
        };

        let contract_info = ContractInfoResponse::new(
            result.code_id,
            Addr::unchecked(result.creator),
            admin,
            false,
            ibc_port_id,
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

        let checksum = Checksum::try_from(code_info.data_hash.as_slice())
            .map_err(|checksum_error| CwEnvError::StdErr(checksum_error.to_string()))?;

        let c = CodeInfoResponse::new(code_id, Addr::unchecked(code_info.creator), checksum);

        Ok(c)
    }

    fn instantiate2_addr(
        &self,
        code_id: u64,
        creator: &Addr,
        salt: cosmwasm_std::Binary,
    ) -> Result<String, Self::Error> {
        let checksum = self.code_id_hash(code_id)?;

        let account_id = AccountId::from_str(creator.as_str()).unwrap();
        let prefix = account_id.prefix();
        let canon = account_id.to_bytes();
        let addr =
            instantiate2_address(checksum.as_slice(), &CanonicalAddr::from(canon), &salt).unwrap();

        Ok(AccountId::new(prefix, &addr).unwrap().to_string())
    }

    fn local_hash<T: Uploadable + ContractInstance<Self::Chain>>(
        &self,
        _contract: &T,
    ) -> Result<Checksum, CwEnvError> {
        <T as Uploadable>::wasm(&MOCK_CHAIN_INFO.into()).checksum()
    }
}
