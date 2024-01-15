use std::str::FromStr;

use crate::contract::WasmPath;
use crate::prelude::Uploadable;
use cosmwasm_std::{Addr, StdResult};

use cw_orch_core::environment::queriers::bank::{BankQuerier, BankQuerierGetter};
use cw_orch_core::environment::BankSetter;
use cw_orch_traits::stargate::Stargate;

use cosmwasm_std::{Binary, Coin, Uint128};
use cw_multi_test::AppResponse;
use osmosis_test_tube::{
    Account, Bank, ExecuteResponse, Gamm, Module, Runner, RunnerError, SigningAccount, Wasm,
};

// This should be the way to import stuff.
// But apparently osmosis-test-tube doesn't have the same dependencies as the test-tube package
use osmosis_test_tube::osmosis_std::{
    cosmwasm_to_proto_coins, types::cosmos::bank::v1beta1::MsgSend,
};

use osmosis_test_tube::OsmosisTestApp;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use serde::Serialize;

use crate::{
    environment::TxHandler,
    environment::{ChainState, StateInterface},
    error::CwOrchError,
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
/// let mut tube: OsmosisTestTube = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));
///
/// // create an additional account
/// let account = tube.init_account(coins(1_000_000_000, "uatom")).unwrap();
///
/// // query the balance
/// let balance: Uint128 = tube.query_balance(&account.address(), "uatom").unwrap();
/// assert_eq!(balance.u128(), 1_000_000_000u128);
/// ```
#[derive(Clone)]
pub struct OsmosisTestTube<S: StateInterface = MockState> {
    /// Address used for the operations.
    pub sender: Rc<SigningAccount>,
    /// Inner mutable state storage for contract addresses and code-ids
    pub state: Rc<RefCell<S>>,
    /// Inner mutable cw-multi-test app backend
    pub app: Rc<RefCell<OsmosisTestApp>>,
}

pub(crate) fn map_err(e: RunnerError) -> CwOrchError {
    CwOrchError::StdErr(e.to_string())
}

impl<S: StateInterface> OsmosisTestTube<S> {
    /// Creates an account and sets its balance
    pub fn init_account(
        &mut self,
        amount: Vec<cosmwasm_std::Coin>,
    ) -> Result<Rc<SigningAccount>, CwOrchError> {
        let account = self
            .app
            .borrow()
            .init_account(&amount)
            .map_err(map_err)
            .map(Rc::new)?;

        Ok(account)
    }

    /// Creates accounts and sets their balance
    pub fn init_accounts(
        &mut self,
        amount: Vec<cosmwasm_std::Coin>,
        account_n: u64,
    ) -> Result<Vec<Rc<SigningAccount>>, CwOrchError> {
        let accounts: Vec<_> = self
            .app
            .borrow()
            .init_accounts(&amount, account_n)
            .map_err(map_err)
            .map(|s| s.into_iter().map(Rc::new).collect())?;

        Ok(accounts)
    }

    /// Sends coins a specific address
    pub fn bank_send(
        &self,
        to: String,
        amount: Vec<cosmwasm_std::Coin>,
    ) -> Result<AppResponse, CwOrchError> {
        let send_response = Bank::new(&*self.app.borrow())
            .send(
                MsgSend {
                    from_address: self.sender.address(),
                    to_address: to,
                    amount: cosmwasm_to_proto_coins(amount),
                },
                &self.sender,
            )
            .map_err(map_err)?;

        Ok(AppResponse {
            data: Some(Binary(send_response.raw_data)),
            events: send_response.events,
        })
    }

    /// Creates an osmosis pool (helper)
    pub fn create_pool(&self, liquidity: Vec<Coin>) -> Result<u64, CwOrchError> {
        // create balancer pool with basic configuration
        let pool_id = Gamm::new(&*self.app.borrow())
            .create_basic_pool(&liquidity, &self.sender)
            .unwrap()
            .data
            .pool_id;

        Ok(pool_id)
    }

    /// Query the (bank) balance of a native token for and address.
    /// Returns the amount of the native token.
    pub fn query_balance(&self, address: &str, denom: &str) -> Result<Uint128, CwOrchError> {
        let amount = self
            .bank_querier()
            .balance(address, Some(denom.to_string()))?;
        Ok(amount.first().unwrap().amount)
    }

    /// Fetch all the balances of an address.
    pub fn query_all_balances(
        &self,
        address: &str,
    ) -> Result<Vec<cosmwasm_std::Coin>, CwOrchError> {
        let amount = self.bank_querier().balance(address, None)?;
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
            sender: Rc::new(sender),
            state,
            app,
        }
    }
}

impl<S: StateInterface> ChainState for OsmosisTestTube<S> {
    type Out = Rc<RefCell<S>>;

    fn state(&self) -> Self::Out {
        self.state.clone()
    }
}

// Execute on the test chain, returns test response type
impl<S: StateInterface> TxHandler for OsmosisTestTube<S> {
    type Error = CwOrchError;
    type ContractSource = WasmPath;
    type Response = AppResponse;
    type Sender = Rc<SigningAccount>;

    fn sender(&self) -> Addr {
        Addr::unchecked(self.sender.address())
    }

    fn set_sender(&mut self, sender: Self::Sender) {
        self.sender = sender;
    }

    fn upload(&self, contract: &impl Uploadable) -> Result<Self::Response, CwOrchError> {
        let wasm_contents = std::fs::read(contract.wasm().path())?;
        let upload_response = Wasm::new(&*self.app.borrow())
            .store_code(&wasm_contents, None, &self.sender)
            .map_err(map_err)?;

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
        let execute_response = Wasm::new(&*self.app.borrow())
            .execute(contract_address.as_ref(), exec_msg, coins, &self.sender)
            .map_err(map_err)?;

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
        let instantiate_response = Wasm::new(&*self.app.borrow())
            .instantiate(
                code_id,
                init_msg,
                admin.map(|a| a.to_string()).as_deref(),
                label,
                coins,
                &self.sender,
            )
            .map_err(map_err)?;

        Ok(AppResponse {
            data: Some(Binary(instantiate_response.raw_data)),
            events: instantiate_response.events,
        })
    }

    fn migrate<M: Serialize + Debug>(
        &self,
        _migrate_msg: &M,
        _new_code_id: u64,
        _contract_address: &Addr,
    ) -> Result<Self::Response, CwOrchError> {
        panic!("Migrate not implemented on osmosis test_tube")
    }

    fn instantiate2<I: Serialize + Debug>(
        &self,
        _code_id: u64,
        _init_msg: &I,
        _label: Option<&str>,
        _admin: Option<&Addr>,
        _coins: &[cosmwasm_std::Coin],
        _salt: Binary,
        _fix_msg: bool,
    ) -> Result<Self::Response, Self::Error> {
        unimplemented!("Osmosis Test Tube doesn't support Instantiate 2 directly");
    }
}

impl BankSetter for OsmosisTestTube {
    /// It's impossible to set the balance of an address directly in OsmosisTestTub
    /// So for this implementation, we use a weird algorithm
    fn set_balance(
        &mut self,
        _address: &Addr,
        _amount: Vec<Coin>,
    ) -> Result<(), <Self as TxHandler>::Error> {
        // We check the current balance
        unimplemented!();
    }
}

impl Stargate for OsmosisTestTube {
    fn commit_any<R: prost::Message + Default>(
        &self,
        msgs: Vec<prost_types::Any>,
        _memo: Option<&str>,
    ) -> Result<Self::Response, Self::Error> {
        let tx_response: ExecuteResponse<R> = self
            .app
            .borrow()
            .execute_multiple_raw(msgs, &self.sender)
            .map_err(map_err)?;

        Ok(AppResponse {
            data: Some(Binary(tx_response.raw_data)),
            events: tx_response.events,
        })
    }
}

pub(crate) fn to_cosmwasm_coin(
    c: osmosis_std::types::cosmos::base::v1beta1::Coin,
) -> StdResult<Coin> {
    Ok(Coin {
        amount: Uint128::from_str(&c.amount)?,
        denom: c.denom,
    })
}

#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{coins, ContractInfoResponse};

    use cw_orch_core::environment::queriers::{
        bank::{BankQuerier, BankQuerierGetter},
        wasm::{WasmQuerier, WasmQuerierGetter},
    };
    use osmosis_test_tube::Account;

    use super::OsmosisTestTube;
    use counter_contract::{msg::InstantiateMsg, CounterContract};
    use cw_orch::prelude::*;

    #[test]
    fn wasm_querier_works() -> anyhow::Result<()> {
        let app = OsmosisTestTube::new(coins(100_000_000_000_000, "uosmo"));

        let contract = CounterContract::new(app.clone());
        contract.upload()?;
        contract.instantiate(
            &InstantiateMsg { count: 7 },
            Some(&Addr::unchecked(app.sender.address())),
            None,
        )?;

        assert_eq!(
            contract.wasm().checksum()?,
            app.wasm_querier().code_id_hash(contract.code_id()?)?
        );

        let contract_info = app.wasm_querier().contract_info(contract.addr_str()?)?;
        let mut target_contract_info = ContractInfoResponse::default();
        target_contract_info.admin = Some(app.sender.address().to_string());
        target_contract_info.code_id = contract.code_id()?;
        target_contract_info.creator = app.sender.address().to_string();
        target_contract_info.ibc_port = None;
        assert_eq!(contract_info, target_contract_info);

        Ok(())
    }

    #[test]
    fn bank_querier_works() -> anyhow::Result<()> {
        let denom = "urandom";
        let init_coins = coins(45, denom);
        let app = OsmosisTestTube::new(init_coins.clone());
        let sender = app.sender.address();
        assert_eq!(
            app.bank_querier()
                .balance(sender.clone(), Some(denom.to_string()))?,
            init_coins
        );
        assert_eq!(
            app.bank_querier().supply_of(denom.to_string())?,
            init_coins[0]
        );
        Ok(())
    }
}
