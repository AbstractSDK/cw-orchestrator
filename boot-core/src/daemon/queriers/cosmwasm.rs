use crate::{daemon::cosmos_modules, DaemonError};
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
    /// Returns hash of the given contract using the code_id for it
    pub async fn code_id_hash(&self, code_id: u64) -> Result<String, DaemonError> {
        use cosmos_modules::cosmwasm::query_client::*;
        use cosmos_modules::cosmwasm::QueryCodeRequest;
        // query hash of code-id
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryCodeRequest { code_id };
        let resp = client.code(request).await?.into_inner();
        let contract_hash = resp.code_info.unwrap().data_hash;
        let on_chain_hash = base16::encode_lower(&contract_hash);
        Ok(on_chain_hash)
    }

    /// Returns contract information
    pub async fn contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<cosmos_modules::cosmwasm::ContractInfo, DaemonError> {
        use cosmos_modules::cosmwasm::query_client::*;
        use cosmos_modules::cosmwasm::QueryContractInfoRequest;
        // query hash of code-id
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let request = QueryContractInfoRequest {
            address: address.into(),
        };
        let resp = client.contract_info(request).await?.into_inner();
        let contract_info = resp.contract_info.unwrap();
        Ok(contract_info)
    }
}
