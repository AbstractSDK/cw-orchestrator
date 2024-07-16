use crate::{cosmos_modules, cosmos_rpc_query, error::DaemonError, queriers::DaemonQuerier};
use cosmrs::{proto::cosmos::base::query::v1beta1::PageRequest, rpc::HttpClient};

/// Querier for the CosmWasm SDK module
pub struct CosmWasm {
    client: HttpClient,
}

impl DaemonQuerier for CosmWasm {
    fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

impl CosmWasm {
    /// Query code_id by hash
    pub async fn code_id_hash(&self, code_id: u64) -> Result<String, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/Code",
            QueryCodeRequest { code_id: code_id },
            QueryCodeResponse,
        );

        let contract_hash = resp.code_info.unwrap().data_hash;
        let on_chain_hash = base16::encode_lower(&contract_hash);
        Ok(on_chain_hash)
    }

    /// Query contract info
    pub async fn contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<cosmos_modules::cosmwasm::ContractInfo, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/ContractInfo",
            QueryContractInfoRequest {
                address: address.into(),
            },
            QueryContractInfoResponse,
        );

        let contract_info = resp.contract_info.unwrap();
        Ok(contract_info)
    }

    /// Query contract history
    pub async fn contract_history(
        &self,
        address: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::cosmwasm::QueryContractHistoryResponse, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/ContractHistory",
            QueryContractHistoryRequest {
                address: address.into(),
                pagination: pagination,
            },
            QueryContractHistoryResponse,
        );

        Ok(resp)
    }

    /// Query contract state
    pub async fn contract_state(
        &self,
        address: impl Into<String>,
        query_data: Vec<u8>,
    ) -> Result<Vec<u8>, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/SmartContractState",
            QuerySmartContractStateRequest {
                address: address.into(),
                query_data: query_data,
            },
            QuerySmartContractStateResponse,
        );

        Ok(resp.data)
    }

    /// Query all contract state
    pub async fn all_contract_state(
        &self,
        address: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::cosmwasm::QueryAllContractStateResponse, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/AllContractState",
            QueryAllContractStateRequest {
                address: address.into(),
                pagination: pagination,
            },
            QueryAllContractStateResponse,
        );
        Ok(resp)
    }

    /// Query code
    pub async fn code(
        &self,
        code_id: u64,
    ) -> Result<cosmos_modules::cosmwasm::CodeInfoResponse, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/Code",
            QueryCodeRequest { code_id: code_id },
            QueryCodeResponse,
        );

        Ok(resp.code_info.unwrap())
    }

    /// Query code bytes
    pub async fn code_data(&self, code_id: u64) -> Result<Vec<u8>, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/Code",
            QueryCodeRequest { code_id: code_id },
            QueryCodeResponse,
        );

        Ok(resp.data)
    }

    /// Query codes
    pub async fn codes(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<cosmos_modules::cosmwasm::CodeInfoResponse>, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/Codes",
            QueryCodesRequest {
                pagination: pagination
            },
            QueryCodesResponse,
        );

        Ok(resp.code_infos)
    }

    /// Query pinned codes
    pub async fn pinned_codes(
        &self,
    ) -> Result<cosmos_modules::cosmwasm::QueryPinnedCodesResponse, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/PinnedCodes",
            QueryPinnedCodesRequest { pagination: None },
            QueryPinnedCodesResponse,
        );
        Ok(resp)
    }

    /// Query contracts by code
    pub async fn contract_by_codes(
        &self,
        code_id: u64,
    ) -> Result<cosmos_modules::cosmwasm::QueryContractsByCodeResponse, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/ContractsByCode",
            QueryContractsByCodeRequest {
                code_id: code_id,
                pagination: None,
            },
            QueryContractsByCodeResponse,
        );

        Ok(resp)
    }

    /// Query raw contract state
    pub async fn contract_raw_state(
        &self,
        address: impl Into<String>,
        query_data: Vec<u8>,
    ) -> Result<cosmos_modules::cosmwasm::QueryRawContractStateResponse, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/RawContractState",
            QueryRawContractStateRequest {
                address: address.into(),
                query_data: query_data,
            },
            QueryRawContractStateResponse,
        );

        Ok(resp)
    }

    /// Query params
    pub async fn params(
        &self,
    ) -> Result<cosmos_modules::cosmwasm::QueryParamsResponse, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            cosmwasm,
            "/cosmwasm.wasm.v1.Query/Params",
            QueryParamsRequest {},
            QueryParamsResponse,
        );

        Ok(resp)
    }
}
