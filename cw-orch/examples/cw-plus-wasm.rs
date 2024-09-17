use cosmwasm_std::Uint128;
use cw20::BalanceResponse;
use cw20::{Cw20Coin, Cw20ExecuteMsg};
use cw20_base::msg::InstantiateMsg;
use cw20_base::msg::QueryMsg;
use cw_orch::prelude::*;
use file::FileCw20Base;
use networks::LOCAL_JUNO;
use release::ReleaseCw20Base;

pub const INITIAL_AMOUNT: u128 = 567;

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    env_logger::init();

    let daemon = Daemon::builder(LOCAL_JUNO).build()?;

    let release_cw20 = ReleaseCw20Base::new("cw20-test-release", daemon.clone());
    execution(release_cw20, &daemon.sender_addr())?;

    let file_cw20 = FileCw20Base::new("cw20-test-file", daemon.clone());
    execution(file_cw20, &daemon.sender_addr())?;

    Ok(())
}

fn execution<T>(cw20: T, sender: &Addr) -> anyhow::Result<()>
where
    T: Uploadable
        + CwOrchUpload<Daemon>
        + ContractInstance<Daemon>
        + CwOrchInstantiate<Daemon>
        + CwOrchExecute<Daemon>
        + InstantiableContract<InstantiateMsg = InstantiateMsg>
        + ExecutableContract<ExecuteMsg = Cw20ExecuteMsg>
        + QueryableContract<QueryMsg = QueryMsg>,
{
    cw20.upload()?;
    cw20.instantiate(
        &InstantiateMsg {
            name: "cw20".to_string(),
            symbol: "CWTEST".to_string(),
            decimals: 6,
            initial_balances: vec![Cw20Coin {
                address: sender.to_string(),
                amount: Uint128::from(INITIAL_AMOUNT),
            }],
            mint: None,
            marketing: None,
        },
        None,
        &[],
    )?;

    let balance: BalanceResponse = cw20.query(&QueryMsg::Balance {
        address: sender.to_string(),
    })?;

    assert_eq!(balance.balance.u128(), INITIAL_AMOUNT);
    Ok(())
}

pub mod file {
    use super::*;
    #[cw_orch::interface(InstantiateMsg, Cw20ExecuteMsg, QueryMsg, cosmwasm_std::Empty)]
    pub struct FileCw20Base;

    impl<Chain: CwEnv> Uploadable for FileCw20Base<Chain> {
        // Return the path to the wasm file
        fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
            WasmPath::github_file(
                "AbstractSDK",
                "cw-plus",
                "abstract_versions",
                "artifacts/abstract_cw20_base.wasm",
            )
        }
        // Return a CosmWasm contract wrapper
        fn wrapper() -> Box<dyn MockContract<Empty>> {
            Box::new(
                ContractWrapper::new_with_empty(
                    cw20_base::contract::execute,
                    cw20_base::contract::instantiate,
                    cw20_base::contract::query,
                )
                .with_migrate(cw20_base::contract::migrate),
            )
        }
    }
}

pub mod release {
    use super::*;

    // TODO: cw20 Migrate doesn't implement Debug: https://github.com/CosmWasm/cw-plus/pull/910
    #[cw_orch::interface(InstantiateMsg, Cw20ExecuteMsg, QueryMsg, cosmwasm_std::Empty)]
    pub struct ReleaseCw20Base;

    #[cfg(not(target_arch = "wasm32"))]
    impl<Chain: CwEnv> Uploadable for ReleaseCw20Base<Chain> {
        // Return the path to the wasm file
        fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
            WasmPath::github_release("CosmWasm", "cw-plus", "v2.0.0", "cw20_base.wasm")
        }
        // Return a CosmWasm contract wrapper
        fn wrapper() -> Box<dyn MockContract<Empty>> {
            Box::new(
                ContractWrapper::new_with_empty(
                    cw20_base::contract::execute,
                    cw20_base::contract::instantiate,
                    cw20_base::contract::query,
                )
                .with_migrate(cw20_base::contract::migrate),
            )
        }
    }
}
