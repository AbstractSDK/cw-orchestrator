/*
    Daemon tests
*/
#[cfg(test)]
mod contract {
    use speculoos::prelude::*;

    use cosmwasm_std::Uint128;

    use cw20_base::msg::*;

    use crate::{contract, tests::common::common::start_contract};

    #[contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
    pub struct Cw20Base;

    #[test]
    fn general() {
        let (sender, mut contract) = start_contract();

        // upload contract
        let upload_res = contract.upload();
        asserting!("upload is succesful").that(&upload_res).is_ok();

        let code_id = upload_res.unwrap().logs[0].events[1].attributes[1]
            .value
            .clone();

        log::info!("Using code_id {}", code_id);

        // init msg for contract
        let init_msg = cw20_base::msg::InstantiateMsg {
            name: "Token".to_owned(),
            symbol: "TOK".to_owned(),
            decimals: 6u8,
            initial_balances: vec![cw20::Cw20Coin {
                address: sender.to_string(),
                amount: Uint128::from(10000u128),
            }],
            mint: None,
            marketing: None,
        };

        // instantiate contract on chain
        let init_res = contract.instantiate(&init_msg, Some(&sender.clone()), None);
        asserting!("instantiate is successful")
            .that(&init_res)
            .is_ok();

        // do a query and validate its successful
        let query_res = contract.query::<cw20_base::msg::QueryMsg, cw20::BalanceResponse>(
            &cw20_base::msg::QueryMsg::Balance {
                address: sender.to_string(),
            },
        );
        asserting!("query is successful").that(&query_res).is_ok();

        // validate migrations are successful
        let migrate_res = contract.migrate(&MigrateMsg {}, code_id.parse::<u64>().unwrap());
        asserting!("migrate is successful")
            .that(&migrate_res)
            .is_ok();
    }
}
