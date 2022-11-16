use std::{
    cell::RefCell,
    fmt::{self, Debug},
    marker::PhantomData,
    rc::Rc,
};

use crate::{
    error::BootError,
    index_response::IndexResponse,
    state::StateInterface,
    tx_handler::{TxHandler, TxResponse},
};
use cosmwasm_std::{Addr, Coin, CustomQuery, Empty};
use cw_multi_test::Contract as TestContract;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};

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
    /// ID of the contract, used to retrieve addr/code-id
    pub id: String,
    source: ContractCodeReference,
    /// chain object that handles tx execution and queries.
    chain: Chain,
    /// Indicate the type of executemsg
    _execute_msg: PhantomData<E>,
    _instantiate_msg: PhantomData<I>,
    _query_msg: PhantomData<Q>,
    _migrate_msg: PhantomData<M>,
}

#[derive(Default)]
pub struct ContractCodeReference<ExecT = Empty, QueryT = Empty>
where
    ExecT: Clone + fmt::Debug + PartialEq + JsonSchema + DeserializeOwned + 'static,
    QueryT: CustomQuery + DeserializeOwned + 'static,
{
    pub wasm_code_path: Option<String>,
    pub contract_endpoints: Option<Box<dyn TestContract<ExecT, QueryT>>>,
}
/// Expose chain and state function to call them on the contract
impl<
        Chain: TxHandler + Clone,
        E: Serialize + Debug,
        I: Serialize + Debug,
        Q: Serialize + Debug,
        M: Serialize + Debug,
    > Contract<Chain, E, I, Q, M>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(id: impl ToString, chain: &Chain) -> Self {
        Contract {
            id: id.to_string(),
            chain: chain.clone(),
            source: ContractCodeReference::default(),
            _execute_msg: PhantomData,
            _instantiate_msg: PhantomData,
            _query_msg: PhantomData,
            _migrate_msg: PhantomData,
        }
    }

    pub fn chain(&self) -> Chain {
        self.chain.clone()
    }

    pub fn with_wasm_path(mut self, path: impl ToString) -> Self {
        self.source.wasm_code_path = Some(path.to_string());
        self
    }

    pub fn with_mock(mut self, mock_contract: Box<dyn TestContract<Empty, Empty>>) -> Self {
        self.source.contract_endpoints = Some(mock_contract);
        self
    }

    // Chain interfaces
    pub fn execute(&self, msg: &E, coins: Option<&[Coin]>) -> Result<TxResponse<Chain>, BootError> {
        log::info!("executing {}", self.id);
        self.chain
            .execute(msg, coins.unwrap_or(&[]), &self.address()?)
    }
    pub fn instantiate(
        &self,
        msg: &I,
        admin: Option<&Addr>,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        log::info!("instantiating {}", self.id);
        let resp =
            self.chain
                .instantiate(self.code_id()?, msg, None, admin, coins.unwrap_or(&[]))?;
        let contract_address = resp.instantiated_contract_address()?;
        self.set_address(&contract_address);
        log::debug!("instantiate response: {:#?}", resp);
        Ok(resp)
    }
    pub fn upload(&mut self) -> Result<TxResponse<Chain>, BootError> {
        log::info!("uploading {}", self.id);
        let resp = self.chain.upload(&mut self.source)?;
        let code_id = resp.uploaded_code_id()?;
        self.set_code_id(code_id);
        log::debug!("upload response: {:#?}", resp);

        Ok(resp)
    }
    pub fn query<T: Serialize + DeserializeOwned + Debug>(&self, query_msg: &Q) -> Result<T, BootError> {
        log::debug!("Querying {:#?} on {}", query_msg, self.address()?);
        let resp = self.chain.query(query_msg, &self.address()?)?;
        log::info!("{:?}",resp);
        Ok(resp)
    }
    pub fn migrate(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.chain
            .migrate(migrate_msg, new_code_id, &self.address()?)
    }

    // State interfaces
    pub fn address(&self) -> Result<Addr, BootError> {
        self.chain.state().get_address(&self.id)
    }
    pub fn code_id(&self) -> Result<u64, BootError> {
        self.chain.state().get_code_id(&self.id)
    }
    pub fn set_address(&self, address: &Addr) {
        self.chain.state().set_address(&self.id, address)
    }
    pub fn set_code_id(&self, code_id: u64) {
        self.chain.state().set_code_id(&self.id, code_id)
    }
}
