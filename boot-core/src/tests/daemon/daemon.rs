/*
    Contract tests
*/

#[cfg(test)]
mod daemon {
    use cosmwasm_std::{Addr, Uint128};

    use speculoos::prelude::*;

    use crate::tests::common::common::start_contract;

    #[test]
    fn general() {
        let (sender, mut contract) = start_contract();

        // TODO: figure out why this is failing.
        // I think it's related to the tests running at the same time in the same container
        asserting!("address is not present")
            .that(&contract.address())
            .is_err();

        asserting!("upload_if_needed is ok")
            .that(&contract.upload_if_needed())
            .is_ok();

        asserting!("latest_is_uploaded is true")
            .that(&contract.latest_is_uploaded().unwrap())
            .is_true();

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

        let _ = contract.instantiate(&init_msg, Some(&Addr::unchecked(sender)), Some(&vec![]));

        asserting!("address is present")
            .that(&contract.address())
            .is_ok();

        asserting!("migrate_if_needed is none")
            .that(
                &contract
                    .migrate_if_needed(&cw20_base::msg::MigrateMsg {})
                    .unwrap(),
            )
            .is_none();

        asserting!("is_running_latest is true")
            .that(&contract.is_running_latest().unwrap())
            .is_true();

        let _ = contract.upload();

        asserting!("is_running_latest is false")
            .that(&contract.is_running_latest().unwrap())
            .is_false();

        asserting!("migrate_if_needed is some")
            .that(
                &contract
                    .migrate_if_needed(&cw20_base::msg::MigrateMsg {})
                    .unwrap(),
            )
            .is_some();
    }
}
