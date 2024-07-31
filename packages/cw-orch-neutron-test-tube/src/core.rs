pub use neutron_test_tube;

use cosmwasm_std::{coin, Addr, Coins};

use cw_orch_core::contract::interface_traits::Uploadable;
use cw_orch_core::contract::WasmPath;
use cw_orch_core::environment::{BankQuerier, BankSetter, ChainInfo, DefaultQueriers, NetworkInfo};

use cosmwasm_std::{Binary, Coin, Uint128};
use cw_orch_core::CwEnvError;
use cw_orch_mock::cw_multi_test::AppResponse;
use cw_orch_traits::Stargate;
use neutron_test_tube::{
    neutron_std::{cosmwasm_to_proto_coins, types::cosmos::bank::v1beta1::MsgSend},
    Account, Bank, ExecuteResponse, Module, NeutronTestApp, Runner, RunnerError, SigningAccount,
    Wasm,
};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use serde::Serialize;

use cw_orch_core::{
    environment::TxHandler,
    environment::{ChainState, StateInterface},
};

use cw_orch_mock::MockState;

use super::queriers::bank::NeutronTestTubeBankQuerier;

/// Mock Chain info for neutron test tube. This is used to get the right wasm
pub const MOCK_CHAIN_INFO: ChainInfo = ChainInfo {
    chain_id: "neutron-1",
    gas_denom: "untrn",
    gas_price: 0.0,
    grpc_urls: &[],
    lcd_url: None,
    fcd_url: None,
    network_info: NetworkInfo {
        chain_name: "neutron",
        pub_address_prefix: "neutron",
        coin_type: 118u32,
    },
    kind: cw_orch_core::environment::ChainKind::Local,
};

/// Wrapper around a neutron-test-tube [`NeutronTestApp`](neutron_test_tube::NeutronTestApp) backend.
///
/// Stores a local state with a mapping of contract_id -> code_id/address
///
/// The state is customizable by implementing the [`StateInterface`] trait on a custom struct and providing it on the custom constructor.
///
/// ## Example
/// ```
/// # use cosmwasm_std::{Addr, coins, Uint128};
/// use cw_orch_neutron_test_tube::NeutronTestTube;
/// use cw_orch_neutron_test_tube::neutron_test_tube::Account;
///
/// // Creates an app, creates a sender with an initial balance
/// let mut tube: NeutronTestTube = NeutronTestTube::new(coins(1_000_000_000_000, "untrn"));
///
/// // create an additional account
/// let account = tube.init_account(coins(1_000_000_000, "uatom")).unwrap();
///
/// // query the balance
/// let balance: Uint128 = tube.query_balance(&account.address(), "uatom").unwrap();
/// assert_eq!(balance.u128(), 1_000_000_000u128);
/// ```
#[derive(Clone)]
pub struct NeutronTestTube<S: StateInterface = MockState> {
    /// Address used for the operations.
    pub sender: Rc<SigningAccount>,
    /// Inner mutable state storage for contract addresses and code-ids
    pub state: Rc<RefCell<S>>,
    /// Inner mutable cw-multi-test app backend
    pub app: Rc<RefCell<NeutronTestApp>>,
}

pub(crate) fn map_err(e: RunnerError) -> CwEnvError {
    CwEnvError::StdErr(e.to_string())
}

impl<S: StateInterface> NeutronTestTube<S> {
    /// Creates an account and sets its balance
    pub fn init_account(
        &mut self,
        amount: Vec<cosmwasm_std::Coin>,
    ) -> Result<Rc<SigningAccount>, CwEnvError> {
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
    ) -> Result<Vec<Rc<SigningAccount>>, CwEnvError> {
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
    ) -> Result<AppResponse, CwEnvError> {
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
    pub fn query_balance(&self, address: &str, denom: &str) -> Result<Uint128, CwEnvError> {
        let amount = self
            .bank_querier()
            .balance(address, Some(denom.to_string()))?;
        Ok(amount.first().unwrap().amount)
    }

    /// Fetch all the balances of an address.
    pub fn query_all_balances(&self, address: &str) -> Result<Vec<cosmwasm_std::Coin>, CwEnvError> {
        let amount = self.bank_querier().balance(address, None)?;
        Ok(amount)
    }
}

impl NeutronTestTube<MockState> {
    /// Create a mock environment with the default mock state.
    /// init_coins are minted to the sender that is created in the NeutronTestTube environment
    /// Unlike for mocks, the accounts are created by the struct and not provided by the client
    /// Make sure to use only valid bech32 neutron addresses, not mock
    pub fn new(init_coins: Vec<Coin>) -> Self {
        Self::new_custom(init_coins, MockState::new_with_chain_id("neutron-1"))
    }
}

impl<S: StateInterface> NeutronTestTube<S> {
    /// Create a mock environment with a custom mock state.
    /// The state is customizable by implementing the `StateInterface` trait on a custom struct and providing it on the custom constructor.
    pub fn new_custom(init_coins: Vec<Coin>, custom_state: S) -> Self {
        let state = Rc::new(RefCell::new(custom_state));
        let app = Rc::new(RefCell::new(NeutronTestApp::new()));

        let sender = app.borrow().init_account(&init_coins).unwrap();

        Self {
            sender: Rc::new(sender),
            state,
            app,
        }
    }
}

impl<S: StateInterface> ChainState for NeutronTestTube<S> {
    type Out = Rc<RefCell<S>>;

    fn state(&self) -> Self::Out {
        self.state.clone()
    }
}

// Execute on the test chain, returns test response type
impl<S: StateInterface> TxHandler for NeutronTestTube<S> {
    type Error = CwEnvError;
    type ContractSource = WasmPath;
    type Response = AppResponse;
    type Sender = Rc<SigningAccount>;

    fn sender(&self) -> Addr {
        self.sender_addr()
    }

    fn sender_addr(&self) -> Addr {
        Addr::unchecked(self.sender.address())
    }

    fn set_sender(&mut self, sender: Self::Sender) {
        self.sender = sender;
    }

    fn upload<T: Uploadable>(&self, _contract: &T) -> Result<Self::Response, CwEnvError> {
        let wasm_contents = std::fs::read(<T as Uploadable>::wasm(&MOCK_CHAIN_INFO.into()).path())?;
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
    ) -> Result<Self::Response, CwEnvError> {
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
    ) -> Result<Self::Response, CwEnvError> {
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
    ) -> Result<Self::Response, CwEnvError> {
        panic!("Migrate not implemented on neutron test_tube")
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
        unimplemented!("Neutron Test Tube doesn't support Instantiate 2 directly");
    }
}

/// Gas Fee token for NeutronTestTube, used in BankSetter
pub const GAS_TOKEN: &str = "untrn";

impl BankSetter for NeutronTestTube {
    type T = NeutronTestTubeBankQuerier;

    /// It's impossible to set the balance of an address directly in NeutronTestTube
    fn set_balance(
        &mut self,
        _address: impl Into<String>,
        _amount: Vec<Coin>,
    ) -> Result<(), <Self as TxHandler>::Error> {
        unimplemented!();
    }

    fn add_balance(
        &mut self,
        address: impl Into<String>,
        amount: Vec<Coin>,
    ) -> Result<(), <Self as TxHandler>::Error> {
        let mut all_coins: Coins = amount.clone().try_into().unwrap();
        let gas_balance = coin(100_000_000_000_000, GAS_TOKEN);
        all_coins.add(gas_balance).unwrap();

        let new_account = self.init_account(all_coins.into())?;

        self.call_as(&new_account)
            .bank_send(address.into(), amount)?;

        Ok(())

        // We check the current balance
    }
}

impl Stargate for NeutronTestTube {
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

#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{coin, coins, ContractInfoResponse};

    use neutron_test_tube::Account;

    use crate::{GAS_TOKEN, MOCK_CHAIN_INFO};

    use super::NeutronTestTube;
    use counter_contract::{msg::InstantiateMsg, CounterContract};
    use cw_orch::prelude::*;

    #[test]
    fn wasm_querier_works() -> cw_orch::anyhow::Result<()> {
        let app = NeutronTestTube::new(coins(100_000_000_000_000, "untrn"));

        let contract = CounterContract::new(app.clone());
        contract.upload()?;
        contract.instantiate(
            &InstantiateMsg { count: 7 },
            Some(&Addr::unchecked(app.sender.address())),
            None,
        )?;

        assert_eq!(
            CounterContract::<Mock>::wasm(&MOCK_CHAIN_INFO.into()).checksum()?,
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
    fn bank_querier_works() -> cw_orch::anyhow::Result<()> {
        let denom = "urandom";
        let init_coins = coins(45, denom);
        let app = NeutronTestTube::new(init_coins.clone());
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

    #[test]
    fn add_balance_works() -> cw_orch::anyhow::Result<()> {
        let denom = "untrn";
        let init_coins = coins(100_000_000_000_000, denom);
        let mut app = NeutronTestTube::new(init_coins.clone());

        let account = app.init_account(coins(78, "uweird"))?;

        let amount1 = 139823876u128;
        let amount2 = 1398212713563876u128;
        app.add_balance(
            account.address(),
            vec![coin(amount1, GAS_TOKEN), coin(amount2, "uother")],
        )?;

        let balance = app.bank_querier().balance(account.address(), None)?;

        assert_eq!(
            balance,
            vec![
                coin(amount1, GAS_TOKEN),
                coin(amount2, "uother"),
                coin(78, "uweird")
            ]
        );
        Ok(())
    }
}
