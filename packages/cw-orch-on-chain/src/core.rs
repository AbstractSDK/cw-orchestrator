use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use cosmwasm_std::{
    instantiate2_address, to_json_binary, Addr, Api, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    QuerierWrapper, StdError, Storage,
};
use cw_orch_core::environment::{
    BankQuerier, ChainState, DefaultQueriers, EnvironmentQuerier, IndexResponse, NodeQuerier,
    Querier, QuerierGetter, QueryHandler, StateInterface, TxHandler, WasmQuerier,
};
use cw_storage_plus::Item;
use serde::Serialize;

use crate::error::OnChainError;

#[derive(Clone)]
pub enum OnChainDeps<'a> {
    Mut(Rc<RefCell<DepsMut<'a>>>),
    Ref(Rc<RefCell<Deps<'a>>>),
}

impl<'a> From<DepsMut<'a>> for OnChainDeps<'a> {
    fn from(value: DepsMut<'a>) -> Self {
        OnChainDeps::Mut(Rc::new(RefCell::new(value)))
    }
}
impl<'a> From<Deps<'a>> for OnChainDeps<'a> {
    fn from(value: Deps<'a>) -> Self {
        OnChainDeps::Ref(Rc::new(RefCell::new(value)))
    }
}

impl<'a> OnChainDeps<'a> {
    pub fn storage(&self) -> Ref<'_, dyn Storage> {
        match self {
            OnChainDeps::Mut(deps) => Ref::map(deps.borrow(), |deps| deps.storage),
            OnChainDeps::Ref(deps) => Ref::map(deps.borrow(), |deps| deps.storage),
        }
    }

    pub fn storage_mut(&self) -> Result<RefMut<'_, dyn Storage>, StdError> {
        match self {
            OnChainDeps::Mut(deps) => Ok(RefMut::map(deps.borrow_mut(), |deps| deps.storage)),
            OnChainDeps::Ref(_deps) => Err(StdError::generic_err(
                "Can't access storage mut on ref deps",
            )),
        }
    }

    pub fn querier(&self) -> Ref<'_, QuerierWrapper<'a>> {
        match self {
            OnChainDeps::Mut(deps) => Ref::map(deps.borrow(), |deps| &deps.querier),
            OnChainDeps::Ref(deps) => Ref::map(deps.borrow(), |deps| &deps.querier),
        }
    }

    pub fn api(&self) -> Ref<'_, dyn Api> {
        match self {
            OnChainDeps::Mut(deps) => Ref::map(deps.borrow(), |deps| deps.api),
            OnChainDeps::Ref(deps) => Ref::map(deps.borrow(), |deps| deps.api),
        }
    }
}

#[derive(Clone)]
pub struct OnChain<'a> {
    pub env: Env,
    pub deps: OnChainDeps<'a>,
    pub state: Rc<RefCell<HashMap<String, Addr>>>,
}

impl<'a> OnChain<'a> {
    pub fn new(deps: impl Into<OnChainDeps<'a>>, env: &Env) -> OnChain<'a> {
        OnChain {
            env: env.clone(),
            deps: deps.into(),
            state: Rc::new(RefCell::new(HashMap::default())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CwOrchCosmosMsg(pub CosmosMsg);

impl From<CosmosMsg> for CwOrchCosmosMsg {
    fn from(value: CosmosMsg) -> Self {
        Self(value)
    }
}
impl From<CwOrchCosmosMsg> for CosmosMsg {
    fn from(value: CwOrchCosmosMsg) -> Self {
        value.0
    }
}

impl IndexResponse for CwOrchCosmosMsg {
    fn events(&self) -> Vec<cosmwasm_std::Event> {
        unimplemented!()
    }

    fn event_attr_value(
        &self,
        _event_type: &str,
        _attr_key: &str,
    ) -> cosmwasm_std::StdResult<String> {
        unimplemented!()
    }

    fn event_attr_values(&self, _event_type: &str, _attr_key: &str) -> Vec<String> {
        unimplemented!()
    }

    fn data(&self) -> Option<cosmwasm_std::Binary> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct OnChainState<'a> {
    pub deps: OnChainDeps<'a>,
    pub state: Rc<RefCell<HashMap<String, Addr>>>,
}

impl<'a> StateInterface for OnChainState<'a> {
    fn get_address(&self, contract_id: &str) -> Result<Addr, cw_orch_core::CwEnvError> {
        let complete_contract_id = format!("cw-orch-on-chain-{}", contract_id);
        let store = Item::<Addr>::new_dyn(complete_contract_id.to_string());

        self.state
            .borrow()
            .get(&complete_contract_id)
            .cloned()
            .ok_or(StdError::not_found(complete_contract_id))
            .or_else(|_| store.load(&*self.deps.storage()))
            .map_err(Into::into)
    }

    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        let complete_contract_id = format!("cw-orch-on-chain-{}", contract_id);
        let store = Item::<Addr>::new_dyn(complete_contract_id.to_string());

        match self.deps.storage_mut() {
            Ok(mut storage_mut) => {
                store.save(&mut *(storage_mut), address).unwrap();
            }
            Err(_) => {
                self.state
                    .borrow_mut()
                    .insert(complete_contract_id, address.clone());
            }
        }
    }

    fn remove_address(&mut self, _contract_id: &str) {
        todo!()
    }

    fn get_code_id(&self, _contract_id: &str) -> Result<u64, cw_orch_core::CwEnvError> {
        todo!()
    }

    fn set_code_id(&mut self, _contract_id: &str, _code_id: u64) {
        todo!()
    }

    fn remove_code_id(&mut self, _contract_id: &str) {
        todo!()
    }

    fn get_all_addresses(
        &self,
    ) -> Result<std::collections::HashMap<String, Addr>, cw_orch_core::CwEnvError> {
        todo!()
    }

    fn get_all_code_ids(
        &self,
    ) -> Result<std::collections::HashMap<String, u64>, cw_orch_core::CwEnvError> {
        todo!()
    }
}

impl<'a> ChainState for OnChain<'a> {
    type Out = OnChainState<'a>;

    fn state(&self) -> Self::Out {
        OnChainState {
            deps: self.deps.clone(),
            state: self.state.clone(),
        }
    }
}

impl<'a> TxHandler for OnChain<'a> {
    type Response = CwOrchCosmosMsg;

    type Error = OnChainError;

    type ContractSource = ();

    type Sender = Addr;

    fn sender(&self) -> &Self::Sender {
        &self.env.contract.address
    }

    fn sender_addr(&self) -> Addr {
        self.env.contract.address.clone()
    }

    fn set_sender(&mut self, _sender: Self::Sender) {
        unimplemented!()
    }

    fn upload<T: cw_orch_core::contract::interface_traits::Uploadable>(
        &self,
        _contract_source: &T,
    ) -> Result<Self::Response, Self::Error> {
        unimplemented!("Upload not possible from on-chain")
    }

    fn instantiate<I: Serialize + std::fmt::Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<Self::Response, Self::Error> {
        Ok(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Instantiate {
            admin: admin.map(|a| a.to_string()),
            code_id,
            msg: to_json_binary(init_msg)?,
            funds: coins.to_vec(),
            label: label.unwrap_or("Instantiated with cw-orch").to_string(),
        })
        .into())
    }

    fn instantiate2<I: Serialize + std::fmt::Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
        salt: Binary,
    ) -> Result<Self::Response, Self::Error> {
        Ok(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Instantiate2 {
            admin: admin.map(|a| a.to_string()),
            code_id,
            msg: to_json_binary(init_msg)?,
            funds: coins.to_vec(),
            label: label.unwrap_or("Instantiated with cw-orch").to_string(),
            salt,
        })
        .into())
    }

    fn execute<E: Serialize + std::fmt::Debug>(
        &self,
        exec_msg: &E,
        coins: &[Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error> {
        Ok(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: contract_address.to_string(),
            msg: to_json_binary(exec_msg)?,
            funds: coins.to_vec(),
        })
        .into())
    }

    fn migrate<M: Serialize + std::fmt::Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error> {
        Ok(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Migrate {
            contract_addr: contract_address.to_string(),
            msg: to_json_binary(migrate_msg)?,
            new_code_id,
        })
        .into())
    }
}

impl<'a> QueryHandler for OnChain<'a> {
    type Error = OnChainError;

    fn wait_blocks(&self, _amount: u64) -> Result<(), Self::Error> {
        unimplemented!("You can't manipulate blocks, you're on-chain")
    }

    fn wait_seconds(&self, _secs: u64) -> Result<(), Self::Error> {
        unimplemented!("You can't manipulate time, you're on-chain")
    }

    fn next_block(&self) -> Result<(), Self::Error> {
        unimplemented!("You can't manipulate blocks, you're on-chain")
    }
}

impl<'a> Querier for OnChain<'a> {
    type Error = OnChainError;
}

impl<'a> QuerierGetter<OnChain<'a>> for OnChain<'a> {
    fn querier(&self) -> Self {
        self.clone()
    }
}

impl<'a> DefaultQueriers for OnChain<'a> {
    type Bank = OnChain<'a>;

    type Wasm = OnChain<'a>;

    type Node = OnChain<'a>;
}

impl<'a> BankQuerier for OnChain<'a> {
    fn balance(&self, _address: &Addr, _denom: Option<String>) -> Result<Vec<Coin>, Self::Error> {
        todo!()
    }

    fn total_supply(&self) -> Result<Vec<Coin>, Self::Error> {
        todo!()
    }

    fn supply_of(&self, _denom: impl Into<String>) -> Result<Coin, Self::Error> {
        todo!()
    }
}

impl<'a> WasmQuerier for OnChain<'a> {
    type Chain = OnChain<'a>;

    fn code_id_hash(&self, code_id: u64) -> Result<cosmwasm_std::Checksum, Self::Error> {
        Ok(self.code(code_id)?.checksum)
    }

    fn contract_info(
        &self,
        address: &Addr,
    ) -> Result<cosmwasm_std::ContractInfoResponse, Self::Error> {
        self.deps
            .querier()
            .query_wasm_contract_info(address)
            .map_err(Into::into)
    }

    fn raw_query(&self, address: &Addr, query_keys: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        self.deps
            .querier()
            .query_wasm_raw(address, query_keys)
            .map(|r| r.unwrap_or_default())
            .map_err(Into::into)
    }

    fn smart_query<Q: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        address: &Addr,
        query_msg: &Q,
    ) -> Result<T, Self::Error> {
        self.deps
            .querier()
            .query_wasm_smart(address, query_msg)
            .map_err(Into::into)
    }

    fn code(&self, code_id: u64) -> Result<cosmwasm_std::CodeInfoResponse, Self::Error> {
        self.deps
            .querier()
            .query_wasm_code_info(code_id)
            .map_err(Into::into)
    }

    fn local_hash<
        T: cw_orch_core::contract::interface_traits::Uploadable
            + cw_orch_core::contract::interface_traits::ContractInstance<Self::Chain>,
    >(
        &self,
        _contract: &T,
    ) -> Result<cosmwasm_std::Checksum, cw_orch_core::CwEnvError> {
        unimplemented!()
    }

    fn instantiate2_addr(
        &self,
        code_id: u64,
        creator: &Addr,
        salt: cosmwasm_std::Binary,
    ) -> Result<String, Self::Error> {
        let checksum = self.code_id_hash(code_id)?;
        let creator_canon = self.deps.api().addr_canonicalize(creator.as_str())?;
        let canon = instantiate2_address(checksum.as_slice(), &creator_canon, salt.as_slice())?;
        self.deps
            .api()
            .addr_humanize(&canon)
            .map(|a| a.to_string())
            .map_err(Into::into)
    }
}

impl<'a> EnvironmentQuerier for OnChain<'a> {
    fn env_info(&self) -> cw_orch_core::environment::EnvironmentInfo {
        todo!()
    }
}

impl<'a> NodeQuerier for OnChain<'a> {
    type Response = CwOrchCosmosMsg;

    fn latest_block(&self) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
        todo!()
    }

    fn block_by_height(&self, _height: u64) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
        todo!()
    }

    fn block_height(&self) -> Result<u64, Self::Error> {
        todo!()
    }

    fn block_time(&self) -> Result<u128, Self::Error> {
        todo!()
    }

    fn simulate_tx(&self, _tx_bytes: Vec<u8>) -> Result<u64, Self::Error> {
        todo!()
    }

    fn find_tx(&self, _hash: String) -> Result<Self::Response, Self::Error> {
        todo!()
    }
}
