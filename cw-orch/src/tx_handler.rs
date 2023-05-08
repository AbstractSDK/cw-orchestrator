use crate::{state::ChainState, CwOrcError, IndexResponse, Uploadable};
use cosmwasm_std::{Addr, BlockInfo, Coin, CustomMsg, CustomQuery, Empty};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
// Functions that are callable on the cosmwasm chain/mock
pub type TxResponse<Chain,E=Empty,Q=Empty> = <Chain as TxHandler<E,Q>>::Response;
/// Signer trait for chains.
/// Accesses the sender information from the chain object to perform actions.
pub trait TxHandler<E,Q>: ChainState + Clone 
where 
 E: CustomMsg + DeserializeOwned + 'static,
 Q: CustomQuery + Debug + DeserializeOwned + 'static,

{
    type Response: IndexResponse + Debug;
    type Error: Into<CwOrcError> + Debug;
    type ContractSource;


    // Gets current sender
    fn sender(&self) -> Addr;
    // Skip x amount of blocks
    fn wait_blocks(&self, amount: u64) -> Result<(), Self::Error>;
    fn wait_seconds(&self, secs: u64) -> Result<(), Self::Error>;
    fn next_block(&self) -> Result<(), Self::Error>;
    fn block_info(&self) -> Result<BlockInfo, Self::Error>;
    // Actions //
    fn execute<Exec: Serialize + Debug>(
        &self,
        exec_msg: &Exec,
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
    fn query<Query: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Query,
        contract_address: &Addr,
    ) -> Result<T, Self::Error>;
    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error>;

    fn upload<T>(&self, uploadable: &T) -> Result<Self::Response, Self::Error> where T: Uploadable<CustomExec = E, CustomQuery = Q>;
}

// Required to be a different trait because it can not be implemented for the generic Mock<...>.
// pub trait ChainUpload<ExecC, QueryC>: TxHandler {
//     fn upload(&self, contract_source: &impl Uploadable<E = ExecC,Q = QueryC>) -> Result<Self::Response, Self::Error>;
// }
