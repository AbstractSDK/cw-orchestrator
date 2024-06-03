//! Integration testing execution environment backed by a [cw-multi-test](cw_multi_test) App.
//! It has an associated state that stores deployment information for easy retrieval and contract interactions.

mod core;
pub mod queriers;
mod state;

pub use self::core::CloneTesting;
pub use clone_cw_multi_test as cw_multi_test;
pub use state::MockState;

// We define a new structure to reunite the ContractWrapper objects
mod contract {
    use clone_cw_multi_test::wasm_emulation::storage::storage_wrappers::{
        ReadonlyStorageWrapper, StorageWrapper,
    };
    use clone_cw_multi_test::{
        wasm_emulation::{
            query::{mock_querier::ForkState, MockQuerier},
            storage::dual_std_storage::DualStorage,
        },
        Contract,
    };
    use cosmwasm_std::{Deps, DepsMut, Empty, QuerierWrapper};

    pub struct CloneTestingContract(Box<dyn cw_orch_mock::cw_multi_test::Contract<Empty, Empty>>);

    impl CloneTestingContract {
        pub fn new(contract: Box<dyn cw_orch_mock::cw_multi_test::Contract<Empty, Empty>>) -> Self {
            Self(contract)
        }
    }

    impl Contract<Empty, Empty> for CloneTestingContract {
        fn execute(
            &self,
            deps: cosmwasm_std::DepsMut<cosmwasm_std::Empty>,
            env: cosmwasm_std::Env,
            info: cosmwasm_std::MessageInfo,
            msg: Vec<u8>,
            fork_state: ForkState<Empty, Empty>,
        ) -> anyhow::Result<cosmwasm_std::Response> {
            let querier = MockQuerier::new(fork_state.clone());
            let mut storage = DualStorage::new(
                fork_state.remote,
                env.contract.address.to_string(),
                Box::new(StorageWrapper::new(deps.storage)),
            )?;
            let deps = DepsMut {
                storage: &mut storage,
                api: deps.api,
                querier: QuerierWrapper::new(&querier),
            };
            self.0.execute(deps, env, info, msg)
        }

        fn instantiate(
            &self,
            deps: cosmwasm_std::DepsMut<cosmwasm_std::Empty>,
            env: cosmwasm_std::Env,
            info: cosmwasm_std::MessageInfo,
            msg: Vec<u8>,
            fork_state: ForkState<Empty, Empty>,
        ) -> anyhow::Result<cosmwasm_std::Response> {
            let querier = MockQuerier::new(fork_state.clone());
            let mut storage = DualStorage::new(
                fork_state.remote,
                env.contract.address.to_string(),
                Box::new(StorageWrapper::new(deps.storage)),
            )?;
            let deps = DepsMut {
                storage: &mut storage,
                api: deps.api,
                querier: QuerierWrapper::new(&querier),
            };
            self.0.instantiate(deps, env, info, msg)
        }

        fn query(
            &self,
            deps: cosmwasm_std::Deps<cosmwasm_std::Empty>,
            env: cosmwasm_std::Env,
            msg: Vec<u8>,
            fork_state: ForkState<Empty, Empty>,
        ) -> anyhow::Result<cosmwasm_std::Binary> {
            let querier = MockQuerier::new(fork_state.clone());
            let storage = DualStorage::new(
                fork_state.remote,
                env.contract.address.to_string(),
                Box::new(ReadonlyStorageWrapper::new(deps.storage)),
            )?;
            let deps = Deps {
                storage: &storage,
                api: deps.api,
                querier: QuerierWrapper::new(&querier),
            };
            self.0.query(deps, env, msg)
        }

        fn sudo(
            &self,
            deps: cosmwasm_std::DepsMut<cosmwasm_std::Empty>,
            env: cosmwasm_std::Env,
            msg: Vec<u8>,
            fork_state: ForkState<Empty, Empty>,
        ) -> anyhow::Result<cosmwasm_std::Response> {
            let querier = MockQuerier::new(fork_state.clone());
            let mut storage = DualStorage::new(
                fork_state.remote,
                env.contract.address.to_string(),
                Box::new(StorageWrapper::new(deps.storage)),
            )?;
            let deps = DepsMut {
                storage: &mut storage,
                api: deps.api,
                querier: QuerierWrapper::new(&querier),
            };
            self.0.sudo(deps, env, msg)
        }

        fn reply(
            &self,
            deps: cosmwasm_std::DepsMut<cosmwasm_std::Empty>,
            env: cosmwasm_std::Env,
            msg: cosmwasm_std::Reply,
            fork_state: ForkState<Empty, Empty>,
        ) -> anyhow::Result<cosmwasm_std::Response> {
            let querier = MockQuerier::new(fork_state.clone());
            let mut storage = DualStorage::new(
                fork_state.remote,
                env.contract.address.to_string(),
                Box::new(StorageWrapper::new(deps.storage)),
            )?;
            let deps = DepsMut {
                storage: &mut storage,
                api: deps.api,
                querier: QuerierWrapper::new(&querier),
            };
            self.0.reply(deps, env, msg)
        }

        fn migrate(
            &self,
            deps: cosmwasm_std::DepsMut<cosmwasm_std::Empty>,
            env: cosmwasm_std::Env,
            msg: Vec<u8>,
            fork_state: ForkState<Empty, Empty>,
        ) -> anyhow::Result<cosmwasm_std::Response> {
            let querier = MockQuerier::new(fork_state.clone());
            let mut storage = DualStorage::new(
                fork_state.remote,
                env.contract.address.to_string(),
                Box::new(StorageWrapper::new(deps.storage)),
            )?;
            let deps = DepsMut {
                storage: &mut storage,
                api: deps.api,
                querier: QuerierWrapper::new(&querier),
            };
            self.0.migrate(deps, env, msg)
        }
    }
}
