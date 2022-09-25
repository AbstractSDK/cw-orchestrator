use crate::{contract::ContractCodeReference, state::ChainState, BootError};
use cosmwasm_std::{Addr, Coin, Empty};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
// Functions that are callable on the cosmwasm chain/mock
pub type TxResponse<Chain> = <Chain as TxHandler>::Response;
/// Signer trait for chains.
/// Accesses the sender information from the chain object to perform actions.
pub trait TxHandler: ChainState + Clone {
    type Response: Debug;

    // Gets current sender
    fn sender(&self) -> Addr;
    // Actions //
    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, BootError>;
    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<Self::Response, BootError>;
    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, BootError>;
    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, BootError>;
    fn upload(
        &self,
        // Needs to be &mut to allow mock app to take ownership of contract box-reference.
        contract_source: &mut ContractCodeReference<Empty>,
    ) -> Result<Self::Response, BootError>;
}
