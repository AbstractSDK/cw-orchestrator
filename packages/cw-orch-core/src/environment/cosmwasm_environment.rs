//! Transactional traits for execution environments.

use super::{queriers::QueryHandler, ChainState, IndexResponse};
use crate::{contract::interface_traits::Uploadable, error::CwEnvError};
use cosmwasm_std::{Addr, Coin};
use serde::Serialize;
use std::fmt::Debug;

/// Signals a supported execution environment for CosmWasm contracts
pub trait CwEnv: TxHandler + QueryHandler + Clone {}
impl<T: TxHandler + QueryHandler + Clone> CwEnv for T {}

/// Response type for actions on an environment
pub type TxResponse<Chain> = <Chain as TxHandler>::Response;

/// Signer trait for chains.
/// Accesses the sender information from the chain object to perform actions.
pub trait TxHandler: ChainState + Clone {
    /// Response type for transactions on an environment.
    type Response: IndexResponse + Debug + Send + Clone;
    /// Error type for transactions on an environment.
    type Error: Into<CwEnvError> + Debug + std::error::Error + Send + Sync + 'static;
    /// Source type for uploading to the environment.
    type ContractSource;

    type Sender: Clone;

    /// Gets the address of the current wallet used to sign transactions.
    fn sender(&self) -> Addr;

    /// Sets wallet to sign transactions.
    fn set_sender(&mut self, sender: Self::Sender);

    // Actions

    /// Uploads a contract to the chain.
    fn upload(&self, contract_source: &impl Uploadable) -> Result<Self::Response, Self::Error>;

    /// Send a InstantiateMsg to a contract.
    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<Self::Response, Self::Error>;

    /// Send a ExecMsg to a contract.
    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error>;

    /// Send a MigrateMsg to a contract.
    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error>;

    /// Clones the chain with a different sender.
    /// Usually used to call a contract as a different sender.
    fn call_as(&self, sender: &<Self as TxHandler>::Sender) -> Self {
        let mut chain = self.clone();
        chain.set_sender(sender.clone());
        chain
    }
}

// TODO: Perfect test candidate for `trybuild`
#[cfg(test)]
mod tests {
    use cosmwasm_std::Empty;
    use cw_multi_test::AppResponse;

    use crate::environment::StateInterface;

    use super::*;

    #[derive(Clone)]
    struct MockHandler {}

    impl StateInterface for () {
        fn get_address(&self, _contract_id: &str) -> Result<Addr, CwEnvError> {
            unimplemented!()
        }

        fn set_address(&mut self, _contract_id: &str, _address: &Addr) {
            unimplemented!()
        }

        fn get_code_id(&self, _contract_id: &str) -> Result<u64, CwEnvError> {
            unimplemented!()
        }

        fn set_code_id(&mut self, _contract_id: &str, _code_id: u64) {
            unimplemented!()
        }

        fn get_all_addresses(&self) -> Result<std::collections::HashMap<String, Addr>, CwEnvError> {
            unimplemented!()
        }

        fn get_all_code_ids(&self) -> Result<std::collections::HashMap<String, u64>, CwEnvError> {
            unimplemented!()
        }
    }

    impl ChainState for MockHandler {
        type Out = ();

        fn state(&self) -> Self::Out {}
    }

    impl TxHandler for MockHandler {
        type Response = AppResponse;

        type Error = CwEnvError;

        type ContractSource = ();

        type Sender = ();

        fn sender(&self) -> Addr {
            unimplemented!()
        }

        fn set_sender(&mut self, _sender: Self::Sender) {}

        fn upload(
            &self,
            _contract_source: &impl Uploadable,
        ) -> Result<Self::Response, Self::Error> {
            unimplemented!()
        }

        fn instantiate<I: Serialize + Debug>(
            &self,
            _code_id: u64,
            _init_msg: &I,
            _label: Option<&str>,
            _admin: Option<&Addr>,
            _coins: &[cosmwasm_std::Coin],
        ) -> Result<Self::Response, Self::Error> {
            Ok(AppResponse {
                events: vec![],
                data: None,
            })
        }

        fn execute<E: Serialize + Debug>(
            &self,
            _exec_msg: &E,
            _coins: &[Coin],
            _contract_address: &Addr,
        ) -> Result<Self::Response, Self::Error> {
            unimplemented!()
        }

        fn migrate<M: Serialize + Debug>(
            &self,
            _migrate_msg: &M,
            _new_code_id: u64,
            _contract_address: &Addr,
        ) -> Result<Self::Response, Self::Error> {
            unimplemented!()
        }
    }

    fn associated_error<T: TxHandler>(t: T) -> anyhow::Result<()> {
        t.instantiate(0, &Empty {}, None, None, &[])?;
        Ok(())
    }

    #[test]
    fn tx_handler_error_usable_on_anyhow() -> anyhow::Result<()> {
        associated_error(MockHandler {})?;
        Ok(())
    }
}
