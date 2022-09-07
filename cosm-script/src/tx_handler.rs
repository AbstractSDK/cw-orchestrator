use cosmwasm_std::{Coin, Addr};
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;
use crate::{CosmScriptError, contract::ContractCodeReference};
// Functions that are callable on the cosmwasm chain/mock
pub type TxResponse<Chain> = <Chain as TxHandler>::Response;
/// Signer trait for chains. 
/// Accesses the sender information from the chain object to perform actions.
pub(crate) trait TxHandler {
    type Response;

    // Actions //
    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, CosmScriptError>;
    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: &str,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<Self::Response, CosmScriptError>;
    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, CosmScriptError>;
    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, CosmScriptError>;
    fn upload(&self, contract_source: ContractCodeReference) -> Result<Self::Response, CosmScriptError>;
}

