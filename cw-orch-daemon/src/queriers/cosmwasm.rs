use std::str::FromStr;
use std::time::Duration;

use crate::service::{DaemonChannel, MyRetryPolicy};
use crate::{cosmos_modules, error::DaemonError, Daemon};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use cosmrs::AccountId;
use cosmwasm_std::{
    from_json, instantiate2_address, to_json_binary, CanonicalAddr, CodeInfoResponse,
    ContractInfoResponse, HexBinary,
};
use cw_orch_core::environment::Environment;
use cw_orch_core::{
    contract::interface_traits::Uploadable,
    environment::{Querier, QuerierGetter, WasmQuerier},
};
use tokio::runtime::Handle;
use tonic::transport::Channel;
use tower::retry::RetryLayer;
use tower::{Service, ServiceBuilder};

/// Querier for the CosmWasm SDK module
/// All the async function are prefixed with `_`
pub struct CosmWasm {
    pub channel: Channel,
    pub rt_handle: Option<Handle>,
}

impl CosmWasm {
    pub fn new(daemon: &Daemon) -> Self {
        Self {
            channel: daemon.channel(),
            rt_handle: Some(daemon.rt_handle.clone()),
        }
    }
    pub fn new_async(channel: Channel) -> Self {
        Self {
            channel,
            rt_handle: None,
        }
    }
}

impl QuerierGetter<CosmWasm> for Daemon {
    fn querier(&self) -> CosmWasm {
        CosmWasm::new(self)
    }
}

impl Querier for CosmWasm {
    type Error = DaemonError;
}

impl CosmWasm {
    /// Query code_id by hash
    pub async fn _code_id_hash(&self, code_id: u64) -> Result<HexBinary, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryCodeRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryCodeRequest { code_id };
        let resp = client.code(request).await?.into_inner();
        let contract_hash = resp.code_info.unwrap().data_hash;
        Ok(contract_hash.into())
    }

    /// Query contract info
    pub async fn _contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<ContractInfoResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryContractInfoRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryContractInfoRequest {
            address: address.into(),
        };
        let resp = client.contract_info(request).await?.into_inner();
        let contract_info = resp.contract_info.unwrap();

        let mut c = ContractInfoResponse::default();
        c.code_id = contract_info.code_id;
        c.creator = contract_info.creator;
        c.admin = if contract_info.admin.is_empty() {
            None
        } else {
            Some(contract_info.admin)
        };
        c.ibc_port = if contract_info.ibc_port_id.is_empty() {
            None
        } else {
            Some(contract_info.ibc_port_id)
        };
        Ok(c)
    }

    /// Query contract history
    pub async fn _contract_history(
        &self,
        address: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::cosmwasm::QueryContractHistoryResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryContractHistoryRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryContractHistoryRequest {
            address: address.into(),
            pagination,
        };
        Ok(client.contract_history(request).await?.into_inner())
    }

    /// Query contract state
    pub async fn _contract_state(
        &self,
        address: impl Into<String>,
        query_data: Vec<u8>,
    ) -> Result<Vec<u8>, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QuerySmartContractStateRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QuerySmartContractStateRequest {
            address: address.into(),
            query_data,
        };
        Ok(client
            .smart_contract_state(request)
            .await?
            .into_inner()
            .data)
    }

    /// Query all contract state
    pub async fn _all_contract_state(
        &self,
        address: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::cosmwasm::QueryAllContractStateResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryAllContractStateRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryAllContractStateRequest {
            address: address.into(),
            pagination,
        };
        Ok(client.all_contract_state(request).await?.into_inner())
    }

    /// Query code
    pub async fn _code(&self, code_id: u64) -> Result<CodeInfoResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryCodeRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryCodeRequest { code_id };
        let response = client.code(request).await?.into_inner().code_info.unwrap();

        Ok(cosmrs_to_cosmwasm_code_info(response))
    }

    /// Query code bytes
    pub async fn _code_data(&self, code_id: u64) -> Result<Vec<u8>, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryCodeRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryCodeRequest { code_id };
        Ok(client.code(request).await?.into_inner().data)
    }

    /// Query codes
    pub async fn _codes(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<CodeInfoResponse>, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryCodesRequest};
        let retry_policy = MyRetryPolicy {
            max_retries: 3, // Maximum number of retries
            backoff: Duration::from_secs(1), // Backoff duration
        };
    
        let retry_layer = RetryLayer::new(retry_policy);

        
        let service = ServiceBuilder::new()
        .layer(retry_layer)
        .service(DaemonChannel::new(self.channel.clone()));
    
        let mut client = QueryClient::new(service);

        let request = QueryCodesRequest { pagination };
        let response = client.codes(request).await?.into_inner().code_infos;

        Ok(response
            .into_iter()
            .map(cosmrs_to_cosmwasm_code_info)
            .collect())
    }

    /// Query pinned codes
    pub async fn _pinned_codes(
        &self,
    ) -> Result<cosmos_modules::cosmwasm::QueryPinnedCodesResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryPinnedCodesRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryPinnedCodesRequest { pagination: None };
        Ok(client.pinned_codes(request).await?.into_inner())
    }

    /// Query contracts by code
    pub async fn _contract_by_codes(
        &self,
        code_id: u64,
    ) -> Result<cosmos_modules::cosmwasm::QueryContractsByCodeResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryContractsByCodeRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryContractsByCodeRequest {
            code_id,
            pagination: None,
        };
        Ok(client.contracts_by_code(request).await?.into_inner())
    }

    /// Query raw contract state
    pub async fn _contract_raw_state(
        &self,
        address: impl Into<String>,
        query_data: Vec<u8>,
    ) -> Result<cosmos_modules::cosmwasm::QueryRawContractStateResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryRawContractStateRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryRawContractStateRequest {
            address: address.into(),
            query_data,
        };
        Ok(client.raw_contract_state(request).await?.into_inner())
    }

    /// Query params
    pub async fn _params(
        &self,
    ) -> Result<cosmos_modules::cosmwasm::QueryParamsResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryParamsRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        Ok(client.params(QueryParamsRequest {}).await?.into_inner())
    }
}

impl WasmQuerier for CosmWasm {
    type Chain = Daemon;
    fn code_id_hash(&self, code_id: u64) -> Result<HexBinary, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._code_id_hash(code_id))
    }

    fn contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<cosmwasm_std::ContractInfoResponse, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._contract_info(address))
    }

    fn raw_query(
        &self,
        address: impl Into<String>,
        query_data: Vec<u8>,
    ) -> Result<Vec<u8>, Self::Error> {
        let response = self
            .rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._contract_raw_state(address, query_data))?;

        Ok(response.data)
    }

    fn smart_query<Q: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        address: impl Into<String>,
        query_data: &Q,
    ) -> Result<T, Self::Error> {
        let response = self
            .rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._contract_state(address, to_json_binary(&query_data)?.to_vec()))?;

        Ok(from_json(response)?)
    }

    fn code(&self, code_id: u64) -> Result<cosmwasm_std::CodeInfoResponse, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._code(code_id))
    }

    fn instantiate2_addr(
        &self,
        code_id: u64,
        creator: impl Into<String>,
        salt: cosmwasm_std::Binary,
    ) -> Result<String, Self::Error> {
        let creator_str = creator.into();
        let account_id = AccountId::from_str(&creator_str)?;
        let prefix = account_id.prefix();
        let canon = account_id.to_bytes();
        let checksum = self.code_id_hash(code_id)?;
        let addr = instantiate2_address(checksum.as_slice(), &CanonicalAddr(canon.into()), &salt)?;

        Ok(AccountId::new(prefix, &addr.0)?.to_string())
    }

    fn local_hash<
        T: cw_orch_core::contract::interface_traits::Uploadable
            + cw_orch_core::contract::interface_traits::ContractInstance<Daemon>,
    >(
        &self,
        contract: &T,
    ) -> Result<HexBinary, cw_orch_core::CwEnvError> {
        <T as Uploadable>::wasm(&contract.environment().daemon.sender.chain_info).checksum()
    }
}

pub fn cosmrs_to_cosmwasm_code_info(
    code_info: cosmrs::proto::cosmwasm::wasm::v1::CodeInfoResponse,
) -> CodeInfoResponse {
    let mut c = CodeInfoResponse::default();
    c.code_id = code_info.code_id;
    c.creator = code_info.creator;
    c.checksum = code_info.data_hash.into();
    c
}
