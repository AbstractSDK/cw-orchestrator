use std::marker::PhantomData;
use std::{cell::RefCell, rc::Rc};

use crate::{core::CloneTestingApp, CloneTesting};
use clone_cw_multi_test::AddressGenerator;
use clone_cw_multi_test::CosmosRouter;
use cosmwasm_std::{instantiate2_address, Api, ContractInfoResponse, HexBinary};
use cw_orch_core::{
    contract::interface_traits::{ContractInstance, Uploadable},
    environment::{Querier, QuerierGetter, StateInterface, WasmQuerier},
    CwEnvError,
};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
pub struct CloneWasmQuerier<S> {
    app: Rc<RefCell<CloneTestingApp>>,
    _state: PhantomData<S>,
}

impl<S: StateInterface> CloneWasmQuerier<S> {
    fn new(mock: &CloneTesting<S>) -> Self {
        Self {
            app: mock.app.clone(),
            _state: PhantomData,
        }
    }
}

impl<S> Querier for CloneWasmQuerier<S> {
    type Error = CwEnvError;
}

impl<S: StateInterface> QuerierGetter<CloneWasmQuerier<S>> for CloneTesting<S> {
    fn querier(&self) -> CloneWasmQuerier<S> {
        CloneWasmQuerier::new(self)
    }
}

impl<S: StateInterface> WasmQuerier for CloneWasmQuerier<S> {
    type Chain = CloneTesting<S>;
    fn code_id_hash(&self, code_id: u64) -> Result<HexBinary, CwEnvError> {
        let code_info = self.app.borrow().wrap().query_wasm_code_info(code_id)?;
        Ok(code_info.checksum)
    }

    /// Returns the code_info structure of the provided contract
    fn contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<ContractInfoResponse, CwEnvError> {
        let info = self.app.borrow().wrap().query_wasm_contract_info(address)?;
        Ok(info)
    }

    fn local_hash<T: Uploadable + ContractInstance<Self::Chain>>(
        &self,
        contract: &T,
    ) -> Result<HexBinary, CwEnvError> {
        // We return the hashed contract-id.
        // This will cause the logic to never re-upload a contract if it has the same contract-id.
        let hash: [u8; 32] = Sha256::digest(contract.id()).into();
        Ok(hash.into())
    }

    fn raw_query(
        &self,
        address: impl Into<String>,
        query_data: Vec<u8>,
    ) -> Result<Vec<u8>, Self::Error> {
        let block = self.app.borrow().block_info();
        Ok(self
            .app
            .borrow()
            .read_module(|router, api, storage| {
                router.query(
                    api,
                    storage,
                    &block,
                    cosmwasm_std::QueryRequest::Wasm(cosmwasm_std::WasmQuery::Raw {
                        contract_addr: address.into(),
                        key: query_data.into(),
                    }),
                )
            })?
            .as_slice()
            .to_vec())
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
            .query_wasm_smart(address.into(), query_data)?)
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

    fn instantiate2_addr(
        &self,
        code_id: u64,
        creator: impl Into<String>,
        salt: cosmwasm_std::Binary,
    ) -> Result<String, Self::Error> {
        // Clone Testing needs mock
        let checksum = self.code_id_hash(code_id)?;
        let canon_creator = self.app.borrow().api().addr_canonicalize(&creator.into())?;
        let canonical_addr = instantiate2_address(checksum.as_slice(), &canon_creator, &salt)?;
        Ok(self
            .app
            .borrow()
            .api()
            .addr_humanize(&canonical_addr)?
            .to_string())
    }
}

impl<S> AddressGenerator for CloneWasmQuerier<S> {}
