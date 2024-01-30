use crate::{cosmos_modules, error::DaemonError, Daemon};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use cosmwasm_std::{from_json, to_json_binary, CodeInfoResponse, ContractInfoResponse};
use cw_orch_core::environment::{Querier, QuerierGetter, WasmQuerier};
use tokio::runtime::Handle;
use tonic::transport::Channel;

use super::DaemonQuerier;

/// Querier for the CosmWasm SDK module
pub struct CosmWasm {
    channel: Channel,
}

impl DaemonQuerier for CosmWasm {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl CosmWasm {
    /// Query code_id by hash
    pub async fn code_id_hash(&self, code_id: u64) -> Result<String, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryCodeRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryCodeRequest { code_id };
        let resp = client.code(request).await?.into_inner();
        let contract_hash = resp.code_info.unwrap().data_hash;
        let on_chain_hash = base16::encode_lower(&contract_hash);
        Ok(on_chain_hash)
    }

    /// Query contract info
    pub async fn contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<cosmos_modules::cosmwasm::ContractInfo, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryContractInfoRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryContractInfoRequest {
            address: address.into(),
        };
        let resp = client.contract_info(request).await?.into_inner();
        let contract_info = resp.contract_info.unwrap();
        Ok(contract_info)
    }

    /// Query contract history
    pub async fn contract_history(
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
    pub async fn contract_state(
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
    pub async fn all_contract_state(
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
    pub async fn code(
        &self,
        code_id: u64,
    ) -> Result<cosmos_modules::cosmwasm::CodeInfoResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryCodeRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryCodeRequest { code_id };
        Ok(client.code(request).await?.into_inner().code_info.unwrap())
    }

    /// Query code bytes
    pub async fn code_data(&self, code_id: u64) -> Result<Vec<u8>, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryCodeRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryCodeRequest { code_id };
        Ok(client.code(request).await?.into_inner().data)
    }

    /// Query codes
    pub async fn codes(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<cosmos_modules::cosmwasm::CodeInfoResponse>, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryCodesRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryCodesRequest { pagination };
        Ok(client.codes(request).await?.into_inner().code_infos)
    }

    /// Query pinned codes
    pub async fn pinned_codes(
        &self,
    ) -> Result<cosmos_modules::cosmwasm::QueryPinnedCodesResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryPinnedCodesRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryPinnedCodesRequest { pagination: None };
        Ok(client.pinned_codes(request).await?.into_inner())
    }

    /// Query contracts by code
    pub async fn contract_by_codes(
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
    pub async fn contract_raw_state(
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
    pub async fn params(
        &self,
    ) -> Result<cosmos_modules::cosmwasm::QueryParamsResponse, DaemonError> {
        use cosmos_modules::cosmwasm::{query_client::*, QueryParamsRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        Ok(client.params(QueryParamsRequest {}).await?.into_inner())
    }
}

pub struct DaemonWasmQuerier {
    channel: Channel,
    rt_handle: Handle,
}

impl DaemonWasmQuerier {
    fn new(daemon: &Daemon) -> Self {
        Self {
            channel: daemon.channel(),
            rt_handle: daemon.rt_handle.clone(),
        }
    }
}

impl QuerierGetter<DaemonWasmQuerier> for Daemon {
    fn querier(&self) -> DaemonWasmQuerier {
        DaemonWasmQuerier::new(self)
    }
}

impl Querier for DaemonWasmQuerier {
    type Error = DaemonError;
}

impl WasmQuerier for DaemonWasmQuerier {
    fn code_id_hash(&self, code_id: u64) -> Result<String, Self::Error> {
        self.rt_handle
            .block_on(CosmWasm::new(self.channel.clone()).code_id_hash(code_id))
    }

    fn contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<cosmwasm_std::ContractInfoResponse, Self::Error> {
        let contract_info = self
            .rt_handle
            .block_on(CosmWasm::new(self.channel.clone()).contract_info(address))?;

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

    fn raw_query(
        &self,
        address: impl Into<String>,
        query_data: Vec<u8>,
    ) -> Result<Vec<u8>, Self::Error> {
        let response = self.rt_handle.block_on(
            CosmWasm::new(self.channel.clone()).contract_raw_state(address, query_data),
        )?;

        Ok(response.data)
    }

    fn smart_query<Q: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        address: impl Into<String>,
        query_data: &Q,
    ) -> Result<T, Self::Error> {
        let response = self.rt_handle.block_on(
            CosmWasm::new(self.channel.clone())
                .contract_state(address, to_json_binary(&query_data)?.to_vec()),
        )?;

        Ok(from_json(response)?)
    }

    fn code(&self, code_id: u64) -> Result<cosmwasm_std::CodeInfoResponse, Self::Error> {
        let response = self
            .rt_handle
            .block_on(CosmWasm::new(self.channel.clone()).code(code_id))?;

        let mut c = CodeInfoResponse::default();
        c.code_id = code_id;
        c.creator = response.creator;
        c.checksum = response.data_hash.into();

        Ok(c)
    }
}
