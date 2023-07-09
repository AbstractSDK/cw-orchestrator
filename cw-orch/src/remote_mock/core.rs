use crate::daemon::queriers::DaemonQuerier;
use crate::daemon::GrpcChannel;
use crate::remote_mock::core::queriers::Node;
use cosmwasm_std::Timestamp;
use cw_remote_test::wasm_emulation::storage::analyzer::StorageAnalyzer;
use cw_remote_test::AppBuilder;
use cw_remote_test::BankKeeper;
use cw_remote_test::FailingModule;
use cw_remote_test::WasmKeeper;
use ibc_chain_registry::chain::ChainData;
use std::{cell::RefCell, fmt::Debug, rc::Rc};
use tokio::runtime::Handle;

use cosmwasm_std::{Addr, Empty, Event, Uint128};
use cw_remote_test::{next_block, AppResponse, BasicApp, Contract, Executor};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    environment::TxHandler,
    error::CwOrchError,
    prelude::*,
    state::{ChainState, StateInterface},
};

use crate::mock::state::MockState;

/// Wrapper around a cw-remote-test [`App`](cw_remote_test::App) backend.
///
/// Stores a local state with a mapping of contract_id -> code_id/address
///
/// The state is customizable by implementing the [`StateInterface`] trait on a custom struct and providing it on the custom constructor.
///
/// ## Example
/// ```
/// # use cosmwasm_std::{Addr, coin, Uint128};
/// use cw_orch::prelude::RemoteMock;
///
/// let sender = Addr::unchecked("sender");
/// let mock: RemoteMock = RemoteMock::new(&sender);
///
/// // set a balance
/// mock.set_balance(&sender, vec![coin(100u128, "token")]).unwrap();
///
/// // query the balance
/// let balance: Uint128 = mock.query_balance(&sender, "token").unwrap();
/// assert_eq!(balance.u128(), 100u128);
/// ```
///
/// ## Example with custom state
/// ```
/// # use cosmwasm_std::{Addr, coin, Uint128};
/// use cw_orch::prelude::{RemoteMock, StateInterface};
/// // We just use the MockState as an example here, but you can implement your own state struct.
/// use cw_orch::mock::MockState as CustomState;
///
/// let sender = Addr::unchecked("sender");
/// let mock: RemoteMock = RemoteMock::new_custom(&sender, CustomState::new());
/// ```
#[derive(Clone)]
pub struct RemoteMock<S: StateInterface = MockState> {
    /// Address used for the operations.
    pub sender: Addr,
    /// Inner mutable state storage for contract addresses and code-ids
    pub state: Rc<RefCell<S>>,
    /// Inner mutable cw-multi-test app backend
    pub app: Rc<RefCell<BasicApp<Empty, Empty>>>,
    /// Distant chain associated with the backend env
    pub chain: ChainData,
}

impl<S: StateInterface> RemoteMock<S> {
    /// Set the bank balance of an address.
    pub fn set_balance(
        &self,
        address: &Addr,
        amount: Vec<cosmwasm_std::Coin>,
    ) -> Result<(), CwOrchError> {
        self.app
            .borrow_mut()
            .init_modules(|router, _, storage| router.bank.init_balance(storage, address, amount))
            .map_err(Into::into)
    }

    /// Set the balance for multiple coins at once.
    pub fn set_balances(
        &self,
        balances: &[(&Addr, &[cosmwasm_std::Coin])],
    ) -> Result<(), CwOrchError> {
        self.app
            .borrow_mut()
            .init_modules(|router, _, storage| -> Result<(), CwOrchError> {
                for (addr, coins) in balances {
                    router.bank.init_balance(storage, addr, coins.to_vec())?;
                }
                Ok(())
            })
    }

    /// Query the (bank) balance of a native token for and address.
    /// Returns the amount of the native token.
    pub fn query_balance(&self, address: &Addr, denom: &str) -> Result<Uint128, CwOrchError> {
        let amount = self
            .app
            .borrow()
            .wrap()
            .query_balance(address, denom)?
            .amount;
        Ok(amount)
    }

    /// Fetch all the balances of an address.
    pub fn query_all_balances(
        &self,
        address: &Addr,
    ) -> Result<Vec<cosmwasm_std::Coin>, CwOrchError> {
        let amount = self.app.borrow().wrap().query_all_balances(address)?;
        Ok(amount)
    }

    /// Storage analysis of all the chanegs since instantiation of the app object
    pub fn analysis(&self) -> StorageAnalyzer {
        StorageAnalyzer::new(&self.app.borrow()).unwrap()
    }
}

impl RemoteMock<MockState> {
    /// Create a mock environment with the default mock state.
    pub fn new(chain: impl Into<ChainData>, rt: &Handle) -> Self {
        RemoteMock::new_custom(chain, rt, MockState::new())
    }
}

impl<S: StateInterface> RemoteMock<S> {
    /// Create a mock environment with a custom mock state.
    /// The state is customizable by implementing the `StateInterface` trait on a custom struct and providing it on the custom constructor.
    pub fn new_custom(chain: impl Into<ChainData>, rt: &Handle, custom_state: S) -> Self {
        let state = Rc::new(RefCell::new(custom_state));
        let chain: ChainData = chain.into();

        let mut wasm = WasmKeeper::<Empty, Empty>::new();
        wasm.set_chain(chain.clone());

        let mut bank = BankKeeper::new();
        bank.set_chain(chain.clone());

        let node_querier = Node::new(
            rt.block_on(GrpcChannel::connect(&chain.apis.grpc, &chain.chain_id))
                .unwrap(),
        );
        let block = rt.block_on(node_querier.latest_block()).unwrap();

        // First we instantiate a new app
        let app = AppBuilder::default()
            .with_wasm::<FailingModule<Empty, Empty, Empty>, _>(wasm)
            .with_chain(chain.clone())
            .with_bank(bank)
            .with_block(cosmwasm_std::BlockInfo {
                height: block.header.height.into(),
                time: Timestamp::from_seconds(
                    block.header.time.unix_timestamp().try_into().unwrap(),
                ),
                chain_id: block.header.chain_id.to_string(),
            });

        let app = Rc::new(RefCell::new(app.build(|_, _, _| {})));
        let sender = app.borrow_mut().next_address();

        Self {
            sender,
            state,
            app,
            chain,
        }
    }
}

impl<S: StateInterface> ChainState for RemoteMock<S> {
    type Out = Rc<RefCell<S>>;

    fn state(&self) -> Self::Out {
        Rc::clone(&self.state)
    }
}

// Execute on the test chain, returns test response type
impl<S: StateInterface> TxHandler for RemoteMock<S> {
    type Response = AppResponse;
    type Error = CwOrchError;
    type ContractSource = Box<dyn Contract<Empty, Empty>>;

    fn sender(&self) -> Addr {
        self.sender.clone()
    }

    fn upload(&self, contract: &impl Uploadable) -> Result<Self::Response, CwOrchError> {
        let wasm_contents = std::fs::read(contract.wasm().path())?;
        let code_id = self.app.borrow_mut().store_code(
            cw_remote_test::wasm_emulation::contract::WasmContract::new_local(
                wasm_contents,
                self.chain.clone(),
            ),
        );
        // add contract code_id to events manually
        let mut event = Event::new("store_code");
        event = event.add_attribute("code_id", code_id.to_string());
        let resp = AppResponse {
            events: vec![event],
            ..Default::default()
        };
        Ok(resp)
    }

    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[cosmwasm_std::Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, CwOrchError> {
        self.app
            .borrow_mut()
            .execute_contract(
                self.sender.clone(),
                contract_address.to_owned(),
                exec_msg,
                coins,
            )
            .map_err(From::from)
    }

    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<Self::Response, CwOrchError> {
        let addr = self.app.borrow_mut().instantiate_contract(
            code_id,
            self.sender.clone(),
            init_msg,
            coins,
            label.unwrap_or("contract_init"),
            admin.map(|a| a.to_string()),
        )?;
        // add contract address to events manually
        let mut event = Event::new("instantiate");
        event = event.add_attribute("_contract_address", addr);
        let resp = AppResponse {
            events: vec![event],
            ..Default::default()
        };
        Ok(resp)
    }

    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, CwOrchError> {
        self.app
            .borrow()
            .wrap()
            .query_wasm_smart(contract_address, query_msg)
            .map_err(From::from)
    }

    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, CwOrchError> {
        self.app
            .borrow_mut()
            .migrate_contract(
                self.sender.clone(),
                contract_address.clone(),
                migrate_msg,
                new_code_id,
            )
            .map_err(From::from)
    }

    fn wait_blocks(&self, amount: u64) -> Result<(), CwOrchError> {
        self.app.borrow_mut().update_block(|b| {
            b.height += amount;
            b.time = b.time.plus_seconds(5 * amount);
        });
        Ok(())
    }

    fn wait_seconds(&self, secs: u64) -> Result<(), CwOrchError> {
        self.app.borrow_mut().update_block(|b| {
            b.time = b.time.plus_seconds(secs);
            b.height += secs / 5;
        });
        Ok(())
    }

    fn next_block(&self) -> Result<(), CwOrchError> {
        self.app.borrow_mut().update_block(next_block);
        Ok(())
    }

    fn block_info(&self) -> Result<cosmwasm_std::BlockInfo, CwOrchError> {
        Ok(self.app.borrow().block_info())
    }
}

impl<T: CwOrchExecute<RemoteMock> + ContractInstance<RemoteMock> + Clone> CallAs<RemoteMock> for T {
    type Sender = Addr;

    fn set_sender(&mut self, sender: &Addr) {
        self.as_instance_mut().chain.sender = sender.clone();
    }

    fn call_as(&self, sender: &Self::Sender) -> Self {
        let mut contract = self.clone();
        contract.set_sender(sender);
        contract
    }
}
