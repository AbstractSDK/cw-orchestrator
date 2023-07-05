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
use osmosis_test_tube::Account;
use osmosis_test_tube::Bank;
use osmosis_test_tube::Module;
use osmosis_test_tube::SigningAccount;
use osmosis_test_tube::Wasm;
use std::str::FromStr;

use osmosis_test_tube::cosmrs::proto::cosmos::bank::v1beta1::{
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

/// Wrapper around a cw-multi-test [`App`](cw_multi_test::App) backend.
///
/// Stores a local state with a mapping of contract_id -> code_id/address
///
/// The state is customizable by implementing the [`StateInterface`] trait on a custom struct and providing it on the custom constructor.
///
/// ## Example
/// ```
/// # use cosmwasm_std::{Addr, coin, Uint128};
/// use cw_orch::prelude::Mock;
///
/// let sender = Addr::unchecked("sender");
/// let mock: Mock = Mock::new(&sender);
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
/// use cw_orch::prelude::{Mock, StateInterface};
/// // We just use the MockState as an example here, but you can implement your own state struct.
/// use cw_orch::mock::MockState as CustomState;
///
/// let sender = Addr::unchecked("sender");
/// let mock: Mock = Mock::new_custom(&sender, CustomState::new());
/// ```
#[derive(Clone)]
pub struct TestTube<S: StateInterface = MockState> {
    /// Address used for the operations.
    pub sender: Rc<RefCell<SigningAccount>>,
    /// Inner mutable state storage for contract addresses and code-ids
    pub state: Rc<RefCell<S>>,
    /// Inner mutable cw-multi-test app backend
    pub app: Rc<RefCell<OsmosisTestApp>>,
}

impl<S: StateInterface> TestTube<S> {
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

impl TestTube<MockState> {
    /// Create a mock environment with the default mock state.
    pub fn new(init_coins: Vec<Coin>) -> Self {
        Self::new_custom(init_coins, MockState::new())
    }
}

impl<S: StateInterface> TestTube<S> {
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

impl<S: StateInterface> ChainState for TestTube<S> {
    type Out = Rc<RefCell<S>>;

    fn state(&self) -> Self::Out {
        Rc::clone(&self.state)
    }
}

// Execute on the test chain, returns test response type
impl<S: StateInterface> TxHandler for TestTube<S> {
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

impl<T: CwOrchExecute<TestTube> + ContractInstance<TestTube> + Clone> CallAs<TestTube> for T {
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

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
        Uint128,
    };
    use cw_multi_test::ContractWrapper;
    use serde::Serialize;
    use speculoos::prelude::*;

    use crate::test_tube::core::*;

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
            cw20_base::msg::QueryMsg::Balance { address } => Ok(to_binary::<Response>(
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
        let sender = &Addr::unchecked(SENDER);
        let recipient = &Addr::unchecked(BALANCE_ADDR);
        let amount = 1000000u128;
        let denom = "uosmo";

        let chain = TestTube::new(sender);

        chain
            .set_balance(recipient, vec![Coin::new(amount, denom)])
            .unwrap();
        let balance = chain.query_balance(recipient, denom).unwrap();

        asserting("address balance amount is correct")
            .that(&amount)
            .is_equal_to(balance.u128());

        asserting("sender is correct")
            .that(sender)
            .is_equal_to(chain.sender());

        let contract_source = Box::new(
            ContractWrapper::new(execute, cw20_base::contract::instantiate, query)
                .with_migrate(cw20_base::contract::migrate),
        );

        let init_res = chain.upload("cw20", contract_source).unwrap();
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
            .instantiate(1, &init_msg, None, Some(sender), &[])
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
        let sender = &Addr::unchecked(SENDER);
        let recipient = &Addr::unchecked(BALANCE_ADDR);
        let amount = 1000000u128;
        let denom = "uosmo";

        let mock_state = Rc::new(RefCell::new(MockState::new()));

        let chain = Mock::<_>::new_custom(sender, mock_state);

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
}
