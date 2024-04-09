use std::sync::Arc;

use crate::contract::WasmPath;
use crate::mock::cw_multi_test::AppResponse;
use crate::prelude::Uploadable;
use cosmwasm_std::Addr;
use cosmwasm_std::{Binary, Coin, Uint128};
use injective_std::shim::{cosmwasm_to_proto_coins, try_proto_to_cosmwasm_coins};
use injective_std::types::cosmos::bank::v1beta1::{
    MsgSend, QueryAllBalancesRequest, QueryBalanceRequest,
};

use injective_test_tube::{Account, Bank, Module, RunnerError, SigningAccount, Wasm};

use injective_test_tube::InjectiveTestApp;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use serde::Serialize;

use crate::{
    environment::TxHandler,
    environment::{ChainState, StateInterface},
    error::CwOrchError,
};

use crate::mock::MockState;

pub use injective_test_tube;

/// Wrapper around a injective-test-tube [`InjectiveTestApp`](injective_test_tube::InjectiveTestApp) backend.
///
/// Stores a local state with a mapping of contract_id -> code_id/address
///
/// The state is customizable by implementing the [`StateInterface`] trait on a custom struct and providing it on the custom constructor.
///
/// ## Example
/// ```
/// # use cosmwasm_std::{Addr, coins, Uint128};
/// use cw_orch::injective_test_tube::InjectiveTestTube;
/// use cw_orch::injective_test_tube::injective_test_tube::Account;
///
/// // Creates an app, creates a sender with an initial balance
/// let mut tube: InjectiveTestTube = InjectiveTestTube::new(coins(1_000_000_000_000, "uosmo"));
///
/// // create an additional account
/// let account = tube.init_account(coins(1_000_000_000, "uatom")).unwrap();
///
/// // query the balance
/// let balance: Uint128 = tube.query_balance(&account.address(), "uatom").unwrap();
/// assert_eq!(balance.u128(), 1_000_000_000u128);
/// ```
#[derive(Clone)]
pub struct InjectiveTestTube<S: StateInterface = MockState> {
    /// Address used for the operations.
    pub sender: Rc<SigningAccount>,
    /// Inner mutable state storage for contract addresses and code-ids
    pub state: Rc<RefCell<S>>,
    /// Inner mutable cw-multi-test app backend
    pub app: Rc<RefCell<InjectiveTestApp>>,
}

/// Maps runner error error to OrchError
pub fn map_err(e: RunnerError) -> CwOrchError {
    CwOrchError::StdErr(e.to_string())
}

impl<S: StateInterface> InjectiveTestTube<S> {
    /// Creates an account and sets its balance
    pub fn init_account(
        &mut self,
        amount: Vec<cosmwasm_std::Coin>,
    ) -> Result<<Self as TxHandler>::Sender, CwOrchError> {
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
    ) -> Result<Vec<Arc<SigningAccount>>, CwOrchError> {
        let accounts: Vec<_> = self
            .app
            .borrow()
            .init_accounts(&amount, account_n)
            .map_err(map_err)
            .map(|s| s.into_iter().map(Arc::new).collect())?;

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

    /// Query the (bank) balance of a native token for and address.
    /// Returns the amount of the native token.
    pub fn query_balance(&self, address: &str, denom: &str) -> Result<Uint128, CwOrchError> {
        let amount = try_proto_to_cosmwasm_coins(
            Bank::new(&*self.app.borrow())
                .query_balance(&QueryBalanceRequest {
                    address: address.to_owned(),
                    denom: denom.to_string(),
                })
                .map_err(map_err)?
                .balance,
        )?
        .first()
        .map(|c| c.amount)
        .unwrap_or(Uint128::zero());
        Ok(amount)
    }

    /// Fetch all the balances of an address.
    pub fn query_all_balances(
        &self,
        address: &str,
    ) -> Result<Vec<cosmwasm_std::Coin>, CwOrchError> {
        let amount = try_proto_to_cosmwasm_coins(
            Bank::new(&*self.app.borrow())
                .query_all_balances(&QueryAllBalancesRequest {
                    address: address.to_owned(),
                    pagination: None,
                })
                .map_err(map_err)?
                .balances,
        )?;
        Ok(amount)
    }
}

impl InjectiveTestTube<MockState> {
    /// Create a mock environment with the default mock state.
    /// init_coins are minted to the sender that is created in the InjectiveTestTube environment
    /// Unlike for mocks, the accounts are created by the struct and not provided by the client
    /// Make sure to use only valid bech32 osmosis addresses, not mock
    pub fn new(init_coins: Vec<Coin>) -> Self {
        Self::new_custom(init_coins, MockState::new())
    }
}

impl<S: StateInterface> InjectiveTestTube<S> {
    /// Create a mock environment with a custom mock state.
    /// The state is customizable by implementing the `StateInterface` trait on a custom struct and providing it on the custom constructor.
    pub fn new_custom(init_coins: Vec<Coin>, custom_state: S) -> Self {
        let state = Rc::new(RefCell::new(custom_state));
        let app = Rc::new(RefCell::new(InjectiveTestApp::new()));

        let sender = app.borrow().init_account(&init_coins).unwrap();

        Self {
            sender: Rc::new(sender),
            state,
            app,
        }
    }
}

impl<S: StateInterface> ChainState for InjectiveTestTube<S> {
    type Out = Rc<RefCell<S>>;

    fn state(&self) -> Self::Out {
        self.state.clone()
    }
}

// Execute on the test chain, returns test response type
impl<S: StateInterface> TxHandler for InjectiveTestTube<S> {
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
    ) -> Result<Self::Response, Self::Error> {
        unimplemented!("Injective Test Tube doesn't support Instantiate 2 directly");
    }
}

#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{coins, ContractInfoResponse};
    use cw_orch_core::environment::BankQuerier;

    use injective_test_tube::Account;

    use super::InjectiveTestTube;
    use counter_contract::{msg::InstantiateMsg, CounterContract};
    use cw_orch::prelude::*;

    #[test]
    fn wasm_querier_works() -> anyhow::Result<()> {
        let app = InjectiveTestTube::new(coins(100_000_000_000_000, "inj"));

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

        let contract_info = app.wasm_querier().contract_info(contract.address()?)?;
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
        let app = InjectiveTestTube::new(init_coins.clone());
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
