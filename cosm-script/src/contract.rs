use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

use cosmwasm_std::{Addr, Coin, Empty};
use cw_multi_test::Contract as TestContract;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    error::CosmScriptError,
    index_response::IndexResponse,
    state::StateInterface,
    tx_handler::{TxHandler, TxResponse},
};

pub type StateReference<S> = Rc<RefCell<S>>;
/// An instance of a contract. Contains references to the execution environment (chain) and a local state (state)
/// The state is used to store contract addresses/code-ids
pub struct Contract<
    Chain: TxHandler,
    E: Serialize + Debug,
    I: Serialize + Debug,
    Q: Serialize,
    M: Serialize,
> where
    TxResponse<Chain>: IndexResponse,
{
    /// Name of the contract, used to retrieve addr/code-id
    pub name: String,
    /// chain object that handles tx execution and queries.
    chain: Chain,
    /// Indicate the type of executemsg
    _execute_msg: PhantomData<E>,
    _instantiate_msg: PhantomData<I>,
    _query_msg: PhantomData<Q>,
    _migrate_msg: PhantomData<M>,
}

pub enum ContractCodeReference<T = Empty> {
    WasmCodePath(&'static str),
    ContractEndpoints(Box<dyn TestContract<T>>),
}

/// Expose chain and state function to call them on the contract
impl<
        Chain: TxHandler,
        E: Serialize + Debug,
        I: Serialize + Debug,
        Q: Serialize + Debug,
        M: Serialize + Debug,
    > Contract<Chain, E, I, Q, M>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: Chain) -> Self {
        Contract {
            name: name.to_string(),
            chain,
            _execute_msg: PhantomData,
            _instantiate_msg: PhantomData,
            _query_msg: PhantomData,
            _migrate_msg: PhantomData,
        }
    }

    // Chain interfaces
    pub fn execute(
        &self,
        msg: &E,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, CosmScriptError> {
        self.chain
            .execute(msg, coins.unwrap_or(&[]), &self.address()?)
    }
    pub fn instantiate(
        &self,
        msg: &I,
        admin: Option<&Addr>,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, CosmScriptError> {
        let resp =
            self.chain
                .instantiate(self.code_id()?, msg, None, admin, coins.unwrap_or(&[]))?;
        let contract_address = resp.instantiated_contract_address()?;
        self.set_address(&contract_address);
        Ok(resp)
    }
    pub fn upload(
        &self,
        contract_source: ContractCodeReference<Empty>,
    ) -> Result<TxResponse<Chain>, CosmScriptError> {
        let resp = self.chain.upload(contract_source)?;
        let code_id = resp.uploaded_code_id()?;
        self.set_code_id(code_id);
        Ok(resp)
    }
    pub fn query<T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
    ) -> Result<T, CosmScriptError> {
        self.chain.query(query_msg, &self.address()?)
    }
    fn migrate(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
    ) -> Result<TxResponse<Chain>, CosmScriptError> {
        self.chain
            .migrate(migrate_msg, new_code_id, &self.address()?)
    }

    // State interfaces
    pub fn address(&self) -> Result<Addr, CosmScriptError> {
        self.chain.state().get_address(&self.name)
    }
    pub fn code_id(&self) -> Result<u64, CosmScriptError> {
        self.chain.state().get_code_id(&self.name)
    }
    fn set_address(&self, address: &Addr) {
        self.chain.state().set_address(&self.name, address)
    }
    fn set_code_id(&self, code_id: u64) {
        self.chain.state().set_code_id(&self.name, code_id)
    }
}
