use cosmwasm_std::{Addr, BlockInfo, Coin};
use serde::{de::DeserializeOwned, Serialize};

use self::{bank::BankQuerier, env::EnvironmentQuerier, node::NodeQuerier, wasm::WasmQuerier};
use crate::CwEnvError;
use std::fmt::Debug;

pub mod bank;
pub mod env;
pub mod node;
pub mod wasm;

/// This trait acts as the high-level trait bound for supported queries on a `CwEnv` environment.
/// It also implements some high-level functionality to make it easy to access.
pub trait QueryHandler: DefaultQueriers {
    type Error: Into<CwEnvError> + Debug + std::error::Error + Send + Sync + 'static;

    /// Wait for an amount of blocks.
    fn wait_blocks(&self, amount: u64) -> Result<(), Self::Error>;

    /// Wait for an amount of seconds.
    fn wait_seconds(&self, secs: u64) -> Result<(), Self::Error>;

    /// Wait for next block.
    fn next_block(&self) -> Result<(), Self::Error>;

    /// Return current block info see [`BlockInfo`].
    fn block_info(&self) -> Result<BlockInfo, <Self::Node as Querier>::Error> {
        self.node_querier().latest_block()
    }

    fn balance(
        &self,
        address: impl Into<String>,
        denom: Option<String>,
    ) -> Result<Vec<Coin>, <Self::Bank as Querier>::Error> {
        self.bank_querier().balance(address, denom)
    }

    /// Send a QueryMsg to a contract.
    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, <Self::Wasm as Querier>::Error> {
        self.wasm_querier().smart_query(contract_address, query_msg)
    }
}

pub trait QuerierGetter<Q: Querier> {
    fn querier(&self) -> Q;
}

pub trait Querier {
    type Error: Into<CwEnvError> + Debug + std::error::Error + Send + Sync + 'static;
}

pub trait DefaultQueriers:
    QuerierGetter<Self::Bank>
    + QuerierGetter<Self::Wasm>
    + QuerierGetter<Self::Node>
    + EnvironmentQuerier
{
    type Bank: BankQuerier;
    type Wasm: WasmQuerier<Chain = Self>;
    type Node: NodeQuerier;

    fn bank_querier(&self) -> Self::Bank {
        self.querier()
    }

    fn wasm_querier(&self) -> Self::Wasm {
        self.querier()
    }

    fn node_querier(&self) -> Self::Node {
        self.querier()
    }
}

#[cfg(test)]
pub mod test {
    use cosmwasm_std::{Binary, Coin};
    use serde::Serialize;

    use crate::{
        environment::{
            ChainState, DefaultQueriers, EnvironmentQuerier, IndexResponse, NodeQuerier,
            StateInterface,
        },
        CwEnvError,
    };

    use super::{bank::BankQuerier, wasm::WasmQuerier, QuerierGetter, QueryHandler};

    impl crate::environment::queriers::Querier for MockQuerier {
        type Error = CwEnvError;
    }

    #[derive(Clone)]
    pub struct MockHandler {}

    #[derive(Clone)]
    pub struct MockHandlerInterface {}

    impl BankQuerier for MockQuerier {
        fn balance(
            &self,
            _address: impl Into<String>,
            _denom: Option<String>,
        ) -> Result<Vec<Coin>, Self::Error> {
            // Returns an empty balance
            Ok(vec![])
        }

        fn total_supply(&self) -> Result<Vec<Coin>, Self::Error> {
            unimplemented!()
        }

        fn supply_of(&self, _denom: impl Into<String>) -> Result<Coin, Self::Error> {
            unimplemented!()
        }
    }
    impl WasmQuerier for MockQuerier {
        type Chain = MockHandler;
        fn code_id_hash(&self, _code_id: u64) -> Result<cosmwasm_std::Checksum, Self::Error> {
            unimplemented!()
        }

        fn contract_info(
            &self,
            _address: impl Into<String>,
        ) -> Result<cosmwasm_std::ContractInfoResponse, Self::Error> {
            unimplemented!()
        }

        fn raw_query(
            &self,
            _address: impl Into<String>,
            _query_keys: Vec<u8>,
        ) -> Result<Vec<u8>, Self::Error> {
            unimplemented!()
        }

        fn smart_query<Q: Serialize, T: serde::de::DeserializeOwned>(
            &self,
            _address: impl Into<String>,
            _query_msg: &Q,
        ) -> Result<T, Self::Error> {
            unimplemented!()
        }

        fn code(&self, _code_id: u64) -> Result<cosmwasm_std::CodeInfoResponse, Self::Error> {
            unimplemented!()
        }

        fn instantiate2_addr(
            &self,
            _code_id: u64,
            _creator: impl Into<String>,
            _salt: cosmwasm_std::Binary,
        ) -> Result<String, Self::Error> {
            unimplemented!()
        }

        fn local_hash<
            T: crate::contract::interface_traits::Uploadable
                + crate::contract::interface_traits::ContractInstance<MockHandler>,
        >(
            &self,
            _contract: &T,
        ) -> Result<cosmwasm_std::Checksum, CwEnvError> {
            unimplemented!()
        }
    }

    impl NodeQuerier for MockQuerier {
        type Response = MockQuerier;

        fn latest_block(&self) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
            unimplemented!()
        }

        fn block_by_height(&self, _height: u64) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
            unimplemented!()
        }

        fn block_height(&self) -> Result<u64, Self::Error> {
            unimplemented!()
        }

        fn block_time(&self) -> Result<u128, Self::Error> {
            unimplemented!()
        }

        fn simulate_tx(&self, _tx_bytes: Vec<u8>) -> Result<u64, Self::Error> {
            unimplemented!()
        }

        fn find_tx(&self, _hash: String) -> Result<Self::Response, Self::Error> {
            unimplemented!()
        }
    }

    #[derive(Clone, Debug)]
    pub struct MockQuerier {}

    impl IndexResponse for MockQuerier {
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

        fn data(&self) -> Option<Binary> {
            unimplemented!()
        }
    }

    impl QuerierGetter<MockQuerier> for MockHandler {
        fn querier(&self) -> MockQuerier {
            MockQuerier {}
        }
    }

    impl EnvironmentQuerier for MockHandler {
        fn env_info(&self) -> crate::environment::EnvironmentInfo {
            unimplemented!()
        }
    }

    impl DefaultQueriers for MockHandler {
        type Bank = MockQuerier;
        type Wasm = MockQuerier;
        type Node = MockQuerier;
    }

    impl QueryHandler for MockHandler {
        type Error = CwEnvError;

        fn wait_blocks(&self, _amount: u64) -> Result<(), Self::Error> {
            Ok(())
        }

        fn wait_seconds(&self, _secs: u64) -> Result<(), Self::Error> {
            unimplemented!()
        }

        fn next_block(&self) -> Result<(), Self::Error> {
            unimplemented!()
        }
    }

    impl ChainState for MockHandler {
        type Out = MockHandlerInterface;

        fn state(&self) -> Self::Out {
            unimplemented!()
        }
    }

    impl StateInterface for MockHandlerInterface {
        fn get_address(&self, _contract_id: &str) -> Result<cosmwasm_std::Addr, CwEnvError> {
            unimplemented!()
        }

        fn set_address(&mut self, _contract_id: &str, _address: &cosmwasm_std::Addr) {
            unimplemented!()
        }

        fn get_code_id(&self, _contract_id: &str) -> Result<u64, CwEnvError> {
            unimplemented!()
        }

        fn set_code_id(&mut self, _contract_id: &str, _code_id: u64) {
            unimplemented!()
        }

        fn get_all_addresses(
            &self,
        ) -> Result<std::collections::HashMap<String, cosmwasm_std::Addr>, CwEnvError> {
            unimplemented!()
        }

        fn get_all_code_ids(&self) -> Result<std::collections::HashMap<String, u64>, CwEnvError> {
            unimplemented!()
        }
    }

    fn associated_querier_error<T: QueryHandler>(t: T) -> anyhow::Result<()> {
        t.bank_querier().balance("anyone".to_string(), None)?;
        t.wait_blocks(7)?;
        Ok(())
    }

    #[test]
    fn query_handler_error_usable_on_anyhow() -> anyhow::Result<()> {
        associated_querier_error(MockHandler {})?;
        Ok(())
    }
}
