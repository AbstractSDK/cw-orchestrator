use super::state::MockState;
use crate::{
    contract::ContractCodeReference,
    state::{ChainState, StateInterface},
    tx_handler::TxHandler,
    BootError, BootExecute, CallAs, ContractInstance,
};
use cosmwasm_std::{Addr, Empty, Event, Uint128};
use cw_multi_test::{next_block, App, AppResponse, BasicApp, Executor};
use serde::{de::DeserializeOwned, Serialize};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub fn instantiate_default_mock_env(
    sender: &Addr,
) -> anyhow::Result<(Rc<RefCell<MockState>>, Mock<MockState>)> {
    let mock_state = Rc::new(RefCell::new(MockState::new()));
    let mock_app = Rc::new(RefCell::new(BasicApp::new(|_, _, _| {})));
    let mock_chain = Mock::new(sender, &mock_state, &mock_app)?;
    Ok((mock_state, mock_chain))
}

pub fn instantiate_custom_mock_env<S: StateInterface>(
    sender: &Addr,
    custom_state: S,
) -> anyhow::Result<(Rc<RefCell<S>>, Mock<S>)> {
    let mock_state = Rc::new(RefCell::new(custom_state));
    let mock_app = Rc::new(RefCell::new(BasicApp::new(|_, _, _| {})));
    let mock_chain = Mock::new(sender, &mock_state, &mock_app)?;
    Ok((mock_state, mock_chain))
}

// Generic mock-chain implementation
// Allows for custom state storage
#[derive(Clone)]
pub struct Mock<S: StateInterface = MockState> {
    pub sender: Addr,
    pub state: Rc<RefCell<S>>,
    pub app: Rc<RefCell<App>>,
}

impl<S: StateInterface> Mock<S> {
    /// set the Bank balance of an address
    pub fn set_balance(
        &self,
        address: &Addr,
        amount: Vec<cosmwasm_std::Coin>,
    ) -> Result<(), BootError> {
        self.app
            .borrow_mut()
            .init_modules(|router, _, storage| router.bank.init_balance(storage, address, amount))
            .map_err(Into::into)
    }

    pub fn set_balances(
        &self,
        balances: &[(&Addr, &[cosmwasm_std::Coin])],
    ) -> Result<(), BootError> {
        self.app
            .borrow_mut()
            .init_modules(|router, _, storage| -> Result<(), BootError> {
                for (addr, coins) in balances {
                    router.bank.init_balance(storage, addr, coins.to_vec())?;
                }
                Ok(())
            })
    }

    /// Query the balance of a native token for and address
    /// Returns the amount of the native token
    pub fn query_balance(&self, address: &Addr, denom: &str) -> Result<Uint128, BootError> {
        let amount = self
            .app
            .borrow()
            .wrap()
            .query_balance(address, denom)?
            .amount;
        Ok(amount)
    }

    /// Query all balances of the address
    /// Returns a vector of coins
    pub fn query_all_balances(&self, address: &Addr) -> Result<Vec<cosmwasm_std::Coin>, BootError> {
        let amount = self.app.borrow().wrap().query_all_balances(address)?;
        Ok(amount)
    }
}

impl<S: StateInterface> Mock<S> {
    pub fn new(
        sender: &Addr,
        state: &Rc<RefCell<S>>,
        app: &Rc<RefCell<App>>,
    ) -> anyhow::Result<Self> {
        let instance = Self {
            sender: sender.clone(),
            state: state.clone(),
            app: app.clone(),
        };
        Ok(instance)
    }
}

impl<S: StateInterface> ChainState for Mock<S> {
    type Out = Rc<RefCell<S>>;

    fn state(&self) -> Self::Out {
        Rc::clone(&self.state)
    }
}

impl<S: StateInterface> StateInterface for Rc<RefCell<S>> {
    fn get_address(&self, contract_id: &str) -> Result<Addr, BootError> {
        self.borrow().get_address(contract_id)
    }

    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        self.borrow_mut().set_address(contract_id, address)
    }

    fn get_code_id(&self, contract_id: &str) -> Result<u64, BootError> {
        self.borrow().get_code_id(contract_id)
    }

    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        self.borrow_mut().set_code_id(contract_id, code_id)
    }

    fn get_all_addresses(&self) -> Result<std::collections::HashMap<String, Addr>, BootError> {
        self.borrow().get_all_addresses()
    }

    fn get_all_code_ids(&self) -> Result<std::collections::HashMap<String, u64>, BootError> {
        self.borrow().get_all_code_ids()
    }
}

// Execute on the test chain, returns test response type
impl<S: StateInterface> TxHandler for Mock<S> {
    type Response = AppResponse;
    type Error = BootError;

    fn sender(&self) -> Addr {
        self.sender.clone()
    }

    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[cosmwasm_std::Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, crate::BootError> {
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
    ) -> Result<Self::Response, crate::BootError> {
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
    ) -> Result<T, crate::BootError> {
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
    ) -> Result<Self::Response, crate::BootError> {
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

    fn upload(
        &self,
        contract_source: &mut ContractCodeReference<Empty>,
    ) -> Result<Self::Response, crate::BootError> {
        // transfer ownership of Boxed app to App
        if let Some(contract) = std::mem::replace(&mut contract_source.contract_endpoints, None) {
            let code_id = self.app.borrow_mut().store_code(contract);
            // add contract code_id to events manually
            let mut event = Event::new("store_code");
            event = event.add_attribute("code_id", code_id.to_string());
            let resp = AppResponse {
                events: vec![event],
                ..Default::default()
            };
            Ok(resp)
        } else {
            Err(BootError::StdErr(
                "Contract reference must be cosm-multi-test contract object.".into(),
            ))
        }
    }

    fn wait_blocks(&self, amount: u64) -> Result<(), BootError> {
        self.app.borrow_mut().update_block(|b| {
            b.height += amount;
            b.time = b.time.plus_seconds(5 * amount);
        });
        Ok(())
    }

    fn wait_seconds(&self, secs: u64) -> Result<(), BootError> {
        self.app.borrow_mut().update_block(|b| {
            b.time = b.time.plus_seconds(secs);
            b.height += secs / 5;
        });
        Ok(())
    }

    fn next_block(&self) -> Result<(), BootError> {
        self.app.borrow_mut().update_block(next_block);
        Ok(())
    }

    fn block_info(&self) -> Result<cosmwasm_std::BlockInfo, BootError> {
        Ok(self.app.borrow().block_info())
    }
}

impl<T: BootExecute<Mock> + ContractInstance<Mock> + Clone> CallAs<Mock> for T {
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

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
        Uint128,
    };
    use cw_multi_test::ContractWrapper;

    use serde::Serialize;
    use speculoos::prelude::*;

    use crate::{mock::core::*, ContractCodeReference, TxHandler};

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

        let (_state, chain) = instantiate_default_mock_env(sender).unwrap();

        chain
            .set_balance(recipient, vec![Coin::new(amount, denom)])
            .unwrap();
        let balance = chain.query_balance(recipient, denom).unwrap();

        asserting("address balance amount is correct")
            .that(&amount)
            .is_equal_to(&balance.into());

        asserting("sender is correct")
            .that(sender)
            .is_equal_to(chain.sender());

        let mut contract_source: ContractCodeReference = ContractCodeReference::default();

        contract_source.contract_endpoints = Some(Box::new(
            ContractWrapper::new(execute, cw20_base::contract::instantiate, query)
                .with_migrate(cw20_base::contract::migrate),
        ));

        let init_res = chain.upload(&mut contract_source).unwrap();
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

        let (_, chain) = instantiate_custom_mock_env(sender, mock_state).unwrap();

        chain
            .set_balances(&[(recipient, &[Coin::new(amount, denom)])])
            .unwrap();

        let balances = chain.query_all_balances(recipient).unwrap();
        asserting("recipient balances length is 1")
            .that(&balances.len())
            .is_equal_to(&1);
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
            .is_equal_to(&mock_state.get_code_id(contract_id).unwrap());

        asserting!("that total code_ids is 1")
            .that(&mock_state.get_all_code_ids().unwrap().len())
            .is_equal_to(&1);

        asserting!("that total addresses is 1")
            .that(&mock_state.get_all_addresses().unwrap().len())
            .is_equal_to(&1);
    }
}
