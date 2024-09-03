use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::coins;
use cosmwasm_std::Addr;
use counter_contract::msg::MigrateMsg;
use cw20::BalanceResponse;
use cw20::Cw20QueryMsg;
use cw_orch::daemon::networks::PHOENIX_1;
use cw_orch::prelude::CallAs;
use cw_orch::prelude::ContractInstance;
use cw_orch::prelude::CwOrchMigrate;
use cw_orch::prelude::CwOrchUpload;
use cw_orch::prelude::Uploadable;
use cw_orch::prelude::*;
use cw_orch_clone_testing::CloneTesting;

use cosmwasm_std::Empty;

mod common;
use common::counter_contract::CounterContract;

/// For those Who don't know, CAVERN PROTOCOL was a money market
#[test]
pub fn cavern_integration_test() -> cw_orch::anyhow::Result<()> {
    pretty_env_logger::init();

    let sender = Addr::unchecked(SENDER);
    let market_addr = Addr::unchecked(MARKET_ADDR);

    // Instantiation of the fork platform is a breeze.
    let mut app = CloneTesting::new(PHOENIX_1)?;
    app.set_sender(sender.clone());

    // We add some funds to the sender, because they need some, for what we are about to do.
    app.set_balance(&sender, coins(10_000_000, CURRENCY))?;

    // We query to verify the state changed
    let response_before = query_a_currency_balance(&app)?;
    log::info!("A currency balance before deposit : {:?}", response_before);

    let market = CavernMarket::new("cavern:money-market", app.clone());
    market.set_address(&market_addr);

    market.deposit_stable(&coins(10_000, CURRENCY))?;

    let response_after = query_a_currency_balance(&app)?;
    log::info!("A currency balance after deposit : {:?}", response_after);

    // We assert the balance has changed when depositing some funds
    assert_ne!(response_before, response_after);

    // From here on, we do funky stuff.

    // 1. We migrate the money-market contract to a counter contract.
    // 2. We test to see that the counter contract is not properly instantiated because it was migrated on top of another contract
    // 3. We migrate back to the old code_id to resume normal operations
    // 4. We dump all the changes we have made locally to inspect what changed

    // 0. We start by saving some useful information for later (admin for migration (1.) + code id for remigration (3.))

    let contract_info = app.wasm_querier().contract_info(&market_addr)?;

    let money_market_admin = Addr::unchecked(contract_info.admin.unwrap());
    let money_market_code_id = contract_info.code_id;

    // 1. We migrate
    let counter_contract = CounterContract::new(app.clone());
    counter_contract.upload().unwrap();
    counter_contract.set_address(&market.address().unwrap());

    counter_contract
        .call_as(&money_market_admin)
        .migrate(
            &MigrateMsg { t: "t".to_string() },
            counter_contract.code_id().unwrap(),
        )
        .unwrap();

    // 2. We see that the state is not correctly initialized on this contract (because it's the wrong code id)
    let err = counter_contract
        .query::<GetCountResponse>(&counter_contract::msg::QueryMsg::GetCount {})
        .unwrap_err();

    if !err
        .to_string()
        .contains("type: counter_contract::state::State;")
    {
        panic!(
            "Error {} should contain counter_contract::state::State not found",
            err
        );
    }

    // 3. Now we migrate back and deposit again to verify migration is possible back and forth (from off-chain to on-chain contracts)
    market
        .call_as(&money_market_admin)
        .migrate(&Empty {}, money_market_code_id)?;
    market.deposit_stable(&coins(10_000, CURRENCY))?;

    // We query to verify the state changed
    let response_after_migration = query_a_currency_balance(&app)?;
    log::info!(
        "A Currency balance after migrate and deposit : {:?}",
        response_after_migration
    );
    assert_ne!(response_after_migration, response_after);

    // 4. We dump all the storage changes to analyze them
    let analysis = app.storage_analysis();
    log::info!(
        "All contracts storage {:?}",
        analysis.all_readable_contract_storage()
    );

    analysis.compare_all_readable_contract_storage();
    analysis.compare_all_balances();

    Ok(())
}

#[cw_orch::interface(Empty, CavernExecuteMsg, Empty, Empty)]
pub struct CavernMarket;

impl Uploadable for CavernMarket<CloneTesting> {}

#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)] // Function generation
pub enum CavernExecuteMsg {
    ////////////////////
    /// User operations
    ////////////////////
    /// Deposit stable asset to get interest
    #[cw_orch(payable)]
    DepositStable {},
}

/// COUNTER CONTRACT MSGs
#[cw_serde]
#[derive(cw_orch::QueryFns)] // Function generation
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
    // GetCount returns the current count of the cousin contract
    #[returns(GetCountResponse)]
    GetCousinCount {},
}

// Custom response for the query
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

const A_CURRENCY: &str = "terra1gwdxyqtu75es0x5l6cd9flqhh87zjtj7qdankayyr0vtt7s9w4ssm7ds8m";
const SENDER: &str = "terra1ytj0hhw39j88qsx4yapsr6ker83jv3aj354gmj";
const MARKET_ADDR: &str = "terra1zqlcp3aty4p4rjv96h6qdascdn953v6crhwedu5vddxjnp349upscluex6";
const CURRENCY: &str = "ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4";

fn query_a_currency_balance(chain: &CloneTesting) -> cw_orch::anyhow::Result<BalanceResponse> {
    Ok(chain.query(
        &Cw20QueryMsg::Balance {
            address: chain.sender.to_string(),
        },
        &Addr::unchecked(A_CURRENCY),
    )?)
}

#[test]
fn query_hash() -> cw_orch::anyhow::Result<()> {
    let app = CloneTesting::new(PHOENIX_1)?;
    let market = CavernMarket::new("cavern:money-market", app.clone());
    let market_addr = Addr::unchecked(MARKET_ADDR);
    market.set_address(&market_addr);
    market.set_code_id(1340);

    app.wasm_querier().code_id_hash(market.code_id()?)?;
    Ok(())
}

#[test]
fn query_contract_info() -> cw_orch::anyhow::Result<()> {
    let app = CloneTesting::new(PHOENIX_1)?;
    let market = CavernMarket::new("cavern:money-market", app.clone());
    let market_addr = Addr::unchecked(MARKET_ADDR);
    market.set_address(&market_addr);

    app.wasm_querier().contract_info(&market.address()?)?;
    Ok(())
}
