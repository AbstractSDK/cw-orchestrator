use crate::{contract::ContractCodeReference, state::ChainState, BootError, IndexResponse};
use cosmwasm_std::{Addr, BlockInfo, Coin, Empty};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
// Functions that are callable on the cosmwasm chain/mock
pub type TxResponse<Chain> = <Chain as TxHandler>::Response;
/// Signer trait for chains.
/// Accesses the sender information from the chain object to perform actions.
pub trait TxHandler: ChainState + Clone {
    type Response: IndexResponse + Debug;
    type Error: Into<BootError> + Debug;

    // Gets current sender
    fn sender(&self) -> Addr;
    // Skip x amount of blocks
    fn wait_blocks(&self, amount: u64) -> Result<(), Self::Error>;
    fn wait_seconds(&self, secs: u64) -> Result<(), Self::Error>;
    fn next_block(&self) -> Result<(), Self::Error>;
    fn block_info(&self) -> Result<BlockInfo, Self::Error>;
    // Actions //
    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error>;
    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<Self::Response, Self::Error>;
    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, Self::Error>;
    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error>;
    fn upload(
        &self,
        // Needs to be &mut to allow mock app to take ownership of contract box-reference.
        contract_source: &mut ContractCodeReference<Empty>,
    ) -> Result<Self::Response, Self::Error>;
}
