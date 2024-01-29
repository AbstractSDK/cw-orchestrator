use std::{cell::RefCell, fmt::Debug, rc::Rc};

use cosmwasm_std::{
    testing::{MockApi, MockStorage},
    Addr, Coin, Empty, Event, Uint128,
};
use cw_multi_test::{
    ibc::IbcSimpleModule, App, AppBuilder, AppResponse, BankKeeper, Contract, DistributionKeeper,
    Executor, FailingModule, GovFailingModule, StakeKeeper, StargateFailing, WasmKeeper,
};
use cw_utils::NativeBalance;
use serde::Serialize;

use cw_orch_core::{
    contract::interface_traits::Uploadable,
    environment::TxHandler,
    environment::{
        queriers::bank::{BankQuerier},
        BankSetter, ChainState, IndexResponse, StateInterface,
    },
    CwEnvError,
};

use super::state::MockState;

pub type MockApp = App<
    BankKeeper,
    MockApi,
    MockStorage,
    FailingModule<Empty, Empty, Empty>,
    WasmKeeper<Empty, Empty>,
    StakeKeeper,
    DistributionKeeper,
    IbcSimpleModule,
    GovFailingModule,
    StargateFailing,
>;

/// Wrapper around a cw-multi-test [`App`](cw_multi_test::App) backend.
///
/// Stores a local state with a mapping of contract_id -> code_id/address
///
/// The state is customizable by implementing the [`StateInterface`] trait on a custom struct and providing it on the custom constructor.
///
/// The addresses used inside this environment are bech32 addresses. For instance, when creating a mock environment
/// let chain = Mock::new("sender");
/// the actual sender address can be generated using
/// let sender_addr = chain.addr_make("sender")
///
/// ## Example
/// ```
/// # use cosmwasm_std::{Addr, coin, Uint128};
/// use cw_orch_mock::Mock;
/// use cw_orch_core::environment::TxHandler;
///
/// let mock: Mock = Mock::new("sender");
///
/// // set a balance
/// mock.set_balance("sender", vec![coin(100u128, "token")]).unwrap();
///
/// // query the balance
/// let balance: Uint128 = mock.query_balance("sender", "token").unwrap();
/// assert_eq!(balance.u128(), 100u128);
/// ```
///
/// ## Example with custom state
/// ```
/// # use cosmwasm_std::{Addr, coin, Uint128};
/// use cw_orch_mock::Mock;
/// use cw_orch_core::environment::StateInterface;
/// // We just use the MockState as an example here, but you can implement your own state struct.
/// use cw_orch_mock::MockState as CustomState;
///
/// let mock: Mock = Mock::new_custom("sender", CustomState::new());
/// ```
#[derive(Clone)]
pub struct Mock<S: StateInterface = MockState> {
    /// Address used for the operations.
    pub sender: Addr,
    /// Inner mutable state storage for contract addresses and code-ids
    pub state: Rc<RefCell<S>>,
    /// Inner mutable cw-multi-test app backend
    pub app: Rc<RefCell<MockApp>>,
}

impl<S: StateInterface> Mock<S> {
    /// Set the bank balance of an address.
    pub fn set_balance(
        &self,
        address: impl Into<String>,
        amount: Vec<cosmwasm_std::Coin>,
    ) -> Result<(), CwEnvError> {
        self.app
            .borrow_mut()
            .init_modules(|router, _, storage| {
                router
                    .bank
                    .init_balance(storage, &Addr::unchecked(address.into()), amount)
            })
            .map_err(Into::into)
    }

    /// Adds the bank balance of an address.
    pub fn add_balance(
        &self,
        address: impl Into<String>,
        amount: Vec<cosmwasm_std::Coin>,
    ) -> Result<(), CwEnvError> {
        let addr = &Addr::unchecked(address.into());
        let b = self.query_all_balances(addr.clone())?;
        let new_amount = NativeBalance(b) + NativeBalance(amount);
        self.app
            .borrow_mut()
            .init_modules(|router, _, storage| {
                router
                    .bank
                    .init_balance(storage, addr, new_amount.into_vec())
            })
            .map_err(Into::into)
    }

    /// Set the balance for multiple coins at once.
    pub fn set_balances(
        &self,
        balances: &[(impl Into<String> + Clone, &[cosmwasm_std::Coin])],
    ) -> Result<(), CwEnvError> {
        self.app
            .borrow_mut()
            .init_modules(|router, _, storage| -> Result<(), CwEnvError> {
                for (addr, coins) in balances {
                    router.bank.init_balance(
                        storage,
                        &Addr::unchecked(addr.clone()),
                        coins.to_vec(),
                    )?;
                }
                Ok(())
            })
    }

    /// Query the (bank) balance of a native token for and address.
    /// Returns the amount of the native token.
    pub fn query_balance(
        &self,
        address: impl Into<String>,
        denom: &str,
    ) -> Result<Uint128, CwEnvError> {
        Ok(self
            .bank_querier()
            .balance(address, Some(denom.to_string()))?
            .first()
            .map(|c| c.amount)
            .unwrap_or_default())
    }

    /// Fetch all the balances of an address.
    pub fn query_all_balances(
        &self,
        address: impl Into<String>,
    ) -> Result<Vec<cosmwasm_std::Coin>, CwEnvError> {
        self.bank_querier().balance(address, None)
    }
}

impl Mock<MockState> {
    /// Create a mock environment with the default mock state.
    pub fn new(sender: impl Into<String>) -> Self {
        Mock::new_custom(sender, MockState::new())
    }

    pub fn new_with_chain_id(sender: impl Into<String>, chain_id: &str) -> Self {
        let chain = Mock::new_custom(sender, MockState::new());
        chain
            .app
            .borrow_mut()
            .update_block(|b| b.chain_id = chain_id.to_string());

        chain
    }

    pub fn with_chain_id(&mut self, chain_id: &str) {
        self.state.borrow_mut().set_chain_id(chain_id);
        self.app
            .borrow_mut()
            .update_block(|b| b.chain_id = chain_id.to_string());
    }
}

impl<S: StateInterface> Mock<S> {
    /// Create a mock environment with a custom mock state.
    /// The state is customizable by implementing the `StateInterface` trait on a custom struct and providing it on the custom constructor.
    pub fn new_custom(sender: impl Into<String>, custom_state: S) -> Self {
        let state = Rc::new(RefCell::new(custom_state));
        let app = Rc::new(RefCell::new(AppBuilder::new_custom().build(|_, _, _| {})));

        Self {
            sender: Addr::unchecked(sender),
            state,
            app,
        }
    }

    /// Upload a custom contract wrapper.
    /// Support for this is limited.
    pub fn upload_custom(
        &self,
        contract_id: &str,
        wrapper: Box<dyn Contract<Empty, Empty>>,
    ) -> Result<AppResponse, CwEnvError> {
        let code_id = self.app.borrow_mut().store_code(wrapper);
        // add contract code_id to events manually
        let mut event = Event::new("store_code");
        event = event.add_attribute("code_id", code_id.to_string());
        let resp = AppResponse {
            events: vec![event],
            ..Default::default()
        };
        let code_id = IndexResponse::uploaded_code_id(&resp)?;
        self.state.borrow_mut().set_code_id(contract_id, code_id);
        Ok(resp)
    }
}

impl<S: StateInterface> ChainState for Mock<S> {
    type Out = Rc<RefCell<S>>;

    fn state(&self) -> Self::Out {
        self.state.clone()
    }
}

// Execute on the test chain, returns test response type
impl<S: StateInterface> TxHandler for Mock<S> {
    type Response = AppResponse;
    type Error = CwEnvError;
    type ContractSource = Box<dyn Contract<Empty, Empty>>;
    type Sender = Addr;

    fn sender(&self) -> Addr {
        self.sender.clone()
    }

    fn set_sender(&mut self, sender: Self::Sender) {
        self.sender = sender;
    }

    fn upload(&self, contract: &impl Uploadable) -> Result<Self::Response, CwEnvError> {
        let code_id = self.app.borrow_mut().store_code(contract.wrapper());
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
    ) -> Result<Self::Response, CwEnvError> {
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
    ) -> Result<Self::Response, CwEnvError> {
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

    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, CwEnvError> {
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
}

impl<S: StateInterface> BankSetter for Mock<S> {
    fn set_balance(
        &mut self,
        address: impl Into<String>,
        amount: Vec<Coin>,
    ) -> Result<(), <Self as TxHandler>::Error> {
        (*self).set_balance(address, amount)
    }
}

#[cfg(test)]
mod test {

    use cosmwasm_std::{
        coins, to_json_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
        StdResult, Uint128,
    };
    use cw_multi_test::ContractWrapper;
    use cw_orch_core::environment::QueryHandler;
    use serde::Serialize;
    use speculoos::prelude::*;

    use crate::core::*;

    const SENDER: &str = "cosmos123";
    const BALANCE_ADDR: &str = "cosmos456";

    #[derive(Debug, Serialize)]
    struct MigrateMsg {}

    fn execute(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: cw20::Cw20ExecuteMsg,
    ) -> Result<Response, cw20_base::ContractError> {
        match msg {
            cw20::Cw20ExecuteMsg::Mint { recipient, amount } => Ok(Response::default()
                .add_attribute("action", "mint")
                .add_attribute("recipient", recipient)
                .add_attribute("amount", amount)),
            _ => unimplemented!(),
        }
    }

    fn query(_deps: Deps, _env: Env, msg: cw20_base::msg::QueryMsg) -> StdResult<Binary> {
        match msg {
            cw20_base::msg::QueryMsg::Balance { address } => Ok(to_json_binary::<Response>(
                &Response::default()
                    .add_attribute("address", address)
                    .add_attribute("balance", String::from("0")),
            )
            .unwrap()),
            _ => unimplemented!(),
        }
    }

    #[test]
    fn mock() {
        let recipient = BALANCE_ADDR;
        let sender = SENDER;
        let chain = Mock::new(sender);
        let amount = 1000000u128;
        let denom = "uosmo";

        chain
            .set_balance(recipient, vec![Coin::new(amount, denom)])
            .unwrap();
        let balance = chain.query_balance(recipient, denom).unwrap();

        asserting("address balance amount is correct")
            .that(&amount)
            .is_equal_to(balance.u128());

        asserting("sender is correct")
            .that(&sender.to_string())
            .is_equal_to(chain.sender().to_string());

        let contract_source = Box::new(
            ContractWrapper::new(execute, cw20_base::contract::instantiate, query)
                .with_migrate(cw20_base::contract::migrate),
        );

        let init_res = chain.upload_custom("cw20", contract_source).unwrap();
        asserting("contract initialized properly")
            .that(&init_res.events[0].attributes[0].value)
            .is_equal_to(&String::from("1"));

        let init_msg = cw20_base::msg::InstantiateMsg {
            name: String::from("Token"),
            symbol: String::from("TOK"),
            decimals: 6u8,
            initial_balances: vec![],
            mint: None,
            marketing: None,
        };
        let init_res = chain
            .instantiate(1, &init_msg, None, Some(&Addr::unchecked(sender)), &[])
            .unwrap();

        let contract_address = Addr::unchecked(&init_res.events[0].attributes[0].value);

        let exec_res = chain
            .execute(
                &cw20_base::msg::ExecuteMsg::Mint {
                    recipient: recipient.to_string(),
                    amount: Uint128::from(100u128),
                },
                &[],
                &contract_address,
            )
            .unwrap();

        asserting("that exect passed on correctly")
            .that(&exec_res.events[1].attributes[1].value)
            .is_equal_to(&String::from("mint"));

        let query_res = chain
            .query::<cw20_base::msg::QueryMsg, Response>(
                &cw20_base::msg::QueryMsg::Balance {
                    address: recipient.to_string(),
                },
                &contract_address,
            )
            .unwrap();

        asserting("that query passed on correctly")
            .that(&query_res.attributes[1].value)
            .is_equal_to(&String::from("0"));

        let migration_res = chain.migrate(&cw20_base::msg::MigrateMsg {}, 1, &contract_address);
        asserting("that migration passed on correctly")
            .that(&migration_res)
            .is_ok();
    }

    #[test]
    fn custom_mock_env() {
        let mock_state = MockState::new();
        let chain = Mock::<_>::new_custom(SENDER, mock_state);

        let recipient = BALANCE_ADDR;
        let amount = 1000000u128;
        let denom = "uosmo";

        chain
            .set_balances(&[(recipient, &[Coin::new(amount, denom)])])
            .unwrap();

        let balances = chain.query_all_balances(recipient).unwrap();
        asserting("recipient balances length is 1")
            .that(&balances.len())
            .is_equal_to(1);
    }

    #[test]
    fn state_interface() {
        let contract_id = "my_contract";
        let code_id = 1u64;
        let address = &Addr::unchecked(BALANCE_ADDR);
        let mut mock_state = Rc::new(RefCell::new(MockState::new()));

        mock_state.set_address(contract_id, address);
        asserting!("that address has been set")
            .that(&address)
            .is_equal_to(&mock_state.get_address(contract_id).unwrap());

        mock_state.set_code_id(contract_id, code_id);
        asserting!("that code_id has been set")
            .that(&code_id)
            .is_equal_to(mock_state.get_code_id(contract_id).unwrap());

        asserting!("that total code_ids is 1")
            .that(&mock_state.get_all_code_ids().unwrap().len())
            .is_equal_to(1);

        asserting!("that total addresses is 1")
            .that(&mock_state.get_all_addresses().unwrap().len())
            .is_equal_to(1);
    }

    #[test]
    fn add_balance() {
        let chain = Mock::new(SENDER);
        let recipient = BALANCE_ADDR;
        let amount = 1000000u128;
        let denom_1 = "uosmo";
        let denom_2 = "osmou";

        chain
            .add_balance(recipient, vec![Coin::new(amount, denom_1)])
            .unwrap();
        chain
            .add_balance(recipient, vec![Coin::new(amount, denom_2)])
            .unwrap();

        let balances = chain.query_all_balances(recipient).unwrap();
        asserting("recipient balances added")
            .that(&balances)
            .contains_all_of(&[&Coin::new(amount, denom_1), &Coin::new(amount, denom_2)])
    }

    #[test]
    fn bank_querier_works() -> Result<(), CwEnvError> {
        let denom = "urandom";
        let init_coins = coins(45, denom);
        let sender = "sender";
        let app = Mock::new(sender);
        app.set_balance(sender, init_coins.clone())?;
        let sender = app.sender.clone();
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
