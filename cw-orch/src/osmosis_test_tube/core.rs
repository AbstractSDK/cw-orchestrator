use crate::interface_traits::CallAs;
use crate::interface_traits::ContractInstance;
use crate::interface_traits::CwOrchExecute;
use crate::interface_traits::Uploadable;
use crate::paths::WasmPath;
use cosmwasm_std::Binary;
use cosmwasm_std::BlockInfo;
use cosmwasm_std::Coin;
use cosmwasm_std::Timestamp;
use cosmwasm_std::Uint128;
use cw_multi_test::AppResponse;
use osmosis_test_tube::osmosis_std::cosmwasm_to_proto_coins;
use osmosis_test_tube::osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
use osmosis_test_tube::Account;
use osmosis_test_tube::Bank;
use osmosis_test_tube::Gamm;
use osmosis_test_tube::Module;
use osmosis_test_tube::SigningAccount;
use osmosis_test_tube::Wasm;
use std::str::FromStr;

use osmosis_test_tube::osmosis_std::types::cosmos::bank::v1beta1::{
    QueryAllBalancesRequest, QueryBalanceRequest,
};
use osmosis_test_tube::OsmosisTestApp;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use cosmwasm_std::Addr;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    environment::TxHandler,
    error::CwOrchError,
    state::{ChainState, StateInterface},
};

use crate::mock::MockState;

pub use osmosis_test_tube;

/// Wrapper around a osmosis-test-tube [`OsmosisTestApp`](osmosis_test_tube::OsmosisTestApp) backend.
///
/// Stores a local state with a mapping of contract_id -> code_id/address
///
/// The state is customizable by implementing the [`StateInterface`] trait on a custom struct and providing it on the custom constructor.
///
/// ## Example
/// ```
/// # use cosmwasm_std::{Addr, coins, Uint128};
/// use cw_orch::osmosis_test_tube::OsmosisTestTube;
/// use cw_orch::osmosis_test_tube::osmosis_test_tube::Account;
///
/// // Creates an app, creates a sender with an initial balance
/// let tube: OsmosisTestTube = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));
///
/// // create an additional account
/// let account = tube.init_account(coins(1_000_000_000, "uatom")).unwrap();
///
/// // query the balance
/// let balance: Uint128 = tube.query_balance(&account.borrow().address(), "uatom").unwrap();
/// assert_eq!(balance.u128(), 1_000_000_000u128);
/// ```
#[derive(Clone)]
pub struct OsmosisTestTube<S: StateInterface = MockState> {
    /// Address used for the operations.
    pub sender: Rc<RefCell<SigningAccount>>,
    /// Inner mutable state storage for contract addresses and code-ids
    pub state: Rc<RefCell<S>>,
    /// Inner mutable cw-multi-test app backend
    pub app: Rc<RefCell<OsmosisTestApp>>,
}

impl<S: StateInterface> OsmosisTestTube<S> {
    /// Creates an account and sets its balance
    pub fn init_account(
        &self,
        amount: Vec<cosmwasm_std::Coin>,
    ) -> Result<Rc<RefCell<SigningAccount>>, CwOrchError> {
        self.app
            .borrow()
            .init_account(&amount)
            .map_err(Into::into)
            .map(|a| Rc::new(RefCell::new(a)))
    }

    /// Creates accounts and sets their balance
    pub fn init_accounts(
        &self,
        amount: Vec<cosmwasm_std::Coin>,
        account_n: u64,
    ) -> Result<Vec<SigningAccount>, CwOrchError> {
        self.app
            .borrow()
            .init_accounts(&amount, account_n)
            .map_err(Into::into)
    }

    /// Sends coins a specific address
    pub fn bank_send(
        &self,
        to: String,
        amount: Vec<cosmwasm_std::Coin>,
    ) -> Result<AppResponse, CwOrchError> {
        let send_response = Bank::new(&*self.app.borrow()).send(
            MsgSend {
                from_address: self.sender.borrow().address(),
                to_address: to,
                amount: cosmwasm_to_proto_coins(amount),
            },
            &self.sender.borrow(),
        )?;

        Ok(AppResponse {
            data: Some(Binary(send_response.raw_data)),
            events: send_response.events,
        })
    }

    /// Creates an osmosis pool (helper)
    pub fn create_pool(&self, liquidity: Vec<Coin>) -> Result<u64, CwOrchError> {
        // create balancer pool with basic configuration
        let pool_id = Gamm::new(&*self.app.borrow())
            .create_basic_pool(&liquidity, &self.sender.borrow())
            .unwrap()
            .data
            .pool_id;

        Ok(pool_id)
    }

    /// Query the (bank) balance of a native token for and address.
    /// Returns the amount of the native token.
    pub fn query_balance(&self, address: &str, denom: &str) -> Result<Uint128, CwOrchError> {
        let amount = Bank::new(&*self.app.borrow())
            .query_balance(&QueryBalanceRequest {
                address: address.to_owned(),
                denom: denom.to_string(),
            })?
            .balance
            .map(|c| Uint128::from_str(&c.amount).unwrap())
            .unwrap_or(Uint128::zero());
        Ok(amount)
    }

    /// Fetch all the balances of an address.
    pub fn query_all_balances(
        &self,
        address: &str,
    ) -> Result<Vec<cosmwasm_std::Coin>, CwOrchError> {
        let amount = Bank::new(&*self.app.borrow())
            .query_all_balances(&QueryAllBalancesRequest {
                address: address.to_owned(),
                pagination: None,
            })?
            .balances
            .into_iter()
            .map(|c| Coin {
                amount: Uint128::from_str(&c.amount).unwrap(),
                denom: c.denom,
            })
            .collect();
        Ok(amount)
    }
}

impl OsmosisTestTube<MockState> {
    /// Create a mock environment with the default mock state.
    /// init_coins are minted to the sender that is created in the OsmosisTestTube environment
    /// Unlike for mocks, the accounts are created by the struct and not provided by the client
    /// Make sure to use only valid bech32 osmosis addresses, not mock
    pub fn new(init_coins: Vec<Coin>) -> Self {
        Self::new_custom(init_coins, MockState::new())
    }
}

impl<S: StateInterface> OsmosisTestTube<S> {
    /// Create a mock environment with a custom mock state.
    /// The state is customizable by implementing the `StateInterface` trait on a custom struct and providing it on the custom constructor.
    pub fn new_custom(init_coins: Vec<Coin>, custom_state: S) -> Self {
        let state = Rc::new(RefCell::new(custom_state));
        let app = Rc::new(RefCell::new(OsmosisTestApp::new()));

        let sender = app.borrow().init_account(&init_coins).unwrap();

        Self {
            sender: Rc::new(RefCell::new(sender)),
            state,
            app,
        }
    }
}

impl<S: StateInterface> ChainState for OsmosisTestTube<S> {
    type Out = Rc<RefCell<S>>;

    fn state(&self) -> Self::Out {
        Rc::clone(&self.state)
    }
}

// Execute on the test chain, returns test response type
impl<S: StateInterface> TxHandler for OsmosisTestTube<S> {
    type Error = CwOrchError;
    type ContractSource = WasmPath;
    type Response = AppResponse;

    fn sender(&self) -> Addr {
        Addr::unchecked(self.sender.borrow().address())
    }

    fn upload(&self, contract: &impl Uploadable) -> Result<Self::Response, CwOrchError> {
        let wasm_contents = std::fs::read(contract.wasm().path())?;
        let upload_response = Wasm::new(&*self.app.borrow()).store_code(
            &wasm_contents,
            None,
            &self.sender.borrow(),
        )?;

        Ok(AppResponse {
            data: Some(Binary(upload_response.raw_data)),
            events: upload_response.events,
        })
    }

    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[cosmwasm_std::Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, CwOrchError> {
        let execute_response = Wasm::new(&*self.app.borrow()).execute(
            contract_address.as_ref(),
            exec_msg,
            coins,
            &self.sender.borrow(),
        )?;

        Ok(AppResponse {
            data: Some(Binary(execute_response.raw_data)),
            events: execute_response.events,
        })
    }

    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<Self::Response, CwOrchError> {
        let instantiate_response = Wasm::new(&*self.app.borrow()).instantiate(
            code_id,
            init_msg,
            admin.map(|a| a.to_string()).as_deref(),
            label,
            coins,
            &self.sender.borrow(),
        )?;

        Ok(AppResponse {
            data: Some(Binary(instantiate_response.raw_data)),
            events: instantiate_response.events,
        })
    }

    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, CwOrchError> {
        let query = Wasm::new(&*self.app.borrow()).query(contract_address.as_ref(), query_msg)?;

        Ok(query)
    }

    fn migrate<M: Serialize + Debug>(
        &self,
        _migrate_msg: &M,
        _new_code_id: u64,
        _contract_address: &Addr,
    ) -> Result<Self::Response, CwOrchError> {
        panic!("Migrate not implemented on osmosis test_tube")
    }

    fn wait_blocks(&self, _amount: u64) -> Result<(), CwOrchError> {
        panic!("Can't wait blocks on osmosis_test_tube")
    }

    fn wait_seconds(&self, secs: u64) -> Result<(), CwOrchError> {
        self.app.borrow().increase_time(secs);
        Ok(())
    }

    fn next_block(&self) -> Result<(), CwOrchError> {
        panic!("Can't wait blocks on osmosis_test_tube")
    }

    fn block_info(&self) -> Result<cosmwasm_std::BlockInfo, CwOrchError> {
        Ok(BlockInfo {
            chain_id: "osmosis-1".to_string(),
            height: self.app.borrow().get_block_height().try_into().unwrap(),
            time: Timestamp::from_nanos(
                self.app.borrow().get_block_time_nanos().try_into().unwrap(),
            ),
        })
    }
}

impl<T: CwOrchExecute<OsmosisTestTube> + ContractInstance<OsmosisTestTube> + Clone>
    CallAs<OsmosisTestTube> for T
{
    type Sender = Rc<RefCell<SigningAccount>>;

    fn set_sender(&mut self, sender: &Rc<RefCell<SigningAccount>>) {
        self.as_instance_mut().chain.sender = sender.clone();
    }

    fn call_as(&self, sender: &Self::Sender) -> Self {
        let mut contract = self.clone();
        contract.set_sender(sender);
        contract
    }
}
