/*
    Mock tests
*/
#[cfg(test)]
mod general {
    use cosmwasm_std::{Addr, Coin, Response, to_binary, DepsMut, Env, MessageInfo, StdResult, Deps, Binary, Uint128};

    use serde::Serialize;
    use speculoos::prelude::*;

    use cw_multi_test::ContractWrapper;

    use crate::{
        mock::core::*,
        TxHandler,
        ContractCodeReference
    };

    const SENDER: &str = "cosmos123";
    const BALANCE_ADDR: &str = "cosmos456";

    #[derive(Debug, Serialize)]
    struct MigrateMsg {}

    fn instantiate(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: cw20_base::msg::InstantiateMsg,
    ) -> StdResult<Response> {
        Ok(Response::default())
    }

    fn execute(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: cw20::Cw20ExecuteMsg,
    ) -> Result<Response, cw20_base::ContractError>
    {
        match msg {
            cw20::Cw20ExecuteMsg::Mint { recipient, amount } => {
                Ok(
                    Response::default()
                        .add_attribute("action", "mint")
                        .add_attribute("recipient", recipient)
                        .add_attribute("amount", amount)
                )
            },
            cw20::Cw20ExecuteMsg::Transfer { recipient: _, amount: _ } => unimplemented!(),
            cw20::Cw20ExecuteMsg::Burn { amount: _ } => unimplemented!(),
            cw20::Cw20ExecuteMsg::Send { contract: _, amount: _, msg: _ } => unimplemented!(),
            cw20::Cw20ExecuteMsg::IncreaseAllowance { spender: _, amount: _, expires: _ } => unimplemented!(),
            cw20::Cw20ExecuteMsg::DecreaseAllowance { spender: _, amount: _, expires: _ } => unimplemented!(),
            cw20::Cw20ExecuteMsg::TransferFrom { owner: _, recipient: _, amount: _ } => unimplemented!(),
            cw20::Cw20ExecuteMsg::SendFrom { owner: _, contract: _, amount: _, msg: _ } => unimplemented!(),
            cw20::Cw20ExecuteMsg::BurnFrom { owner: _, amount: _ } => unimplemented!(),
            cw20::Cw20ExecuteMsg::UpdateMinter { new_minter: _ } => unimplemented!(),
            cw20::Cw20ExecuteMsg::UpdateMarketing { project: _, description: _, marketing: _ } => unimplemented!(),
            cw20::Cw20ExecuteMsg::UploadLogo(_) => unimplemented!(),
        }
    }

    fn query(
        _deps: Deps,
        _env: Env,
        msg: cw20_base::msg::QueryMsg
    ) -> StdResult<Binary> {
        match msg {
            cw20_base::msg::QueryMsg::Balance { address } => {
                Ok(
                    to_binary::<Response>(&Response::default()
                        .add_attribute("address", address)
                        .add_attribute("balance", String::from("0")))
                        .unwrap()
                )
            },
            cw20_base::msg::QueryMsg::TokenInfo {  } => unimplemented!(),
            cw20_base::msg::QueryMsg::Minter {  } => unimplemented!(),
            cw20_base::msg::QueryMsg::Allowance { owner: _, spender: _ } => unimplemented!(),
            cw20_base::msg::QueryMsg::AllAllowances { owner: _, start_after: _, limit: _ } => unimplemented!(),
            cw20_base::msg::QueryMsg::AllSpenderAllowances { spender: _, start_after: _, limit: _ } => unimplemented!(),
            cw20_base::msg::QueryMsg::AllAccounts { start_after: _, limit: _ } => unimplemented!(),
            cw20_base::msg::QueryMsg::MarketingInfo {  } => unimplemented!(),
            cw20_base::msg::QueryMsg::DownloadLogo {  } => unimplemented!(),
        }
    }

    #[test]
    fn mock() {
        let sender = &Addr::unchecked(SENDER);
        let recipient = &Addr::unchecked(BALANCE_ADDR);
        let amount = 1000000u128;
        let denom = "uosmo";

        let mock = instantiate_default_mock_env(sender).unwrap();
        let chain = mock.1;

        chain.set_balance(recipient, vec![Coin::new(amount, denom)]).unwrap();
        let balance = chain.query_balance(recipient, denom).unwrap();

        asserting("address balance amount is correct")
            .that(&amount).is_equal_to(&balance.into());

        asserting("sender is correct")
            .that(sender).is_equal_to(chain.sender());

        let mut contract_source: ContractCodeReference = ContractCodeReference::default();

        contract_source.contract_endpoints = Some(Box::new(
            ContractWrapper::new(execute, instantiate, query)
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
        let init_res = chain.instantiate(1, &init_msg, None, Some(sender), &[]).unwrap();

        let contract_address = Addr::unchecked(&init_res.events[0].attributes[0].value);

        let exec_res = chain.execute(
            &cw20_base::msg::ExecuteMsg::Mint {
                recipient: recipient.to_string(),
                amount: Uint128::from(100u128)
            },
            &[],
            &contract_address)
            .unwrap();

        asserting("that exect passed on correctly")
            .that(&exec_res.events[1].attributes[1].value)
            .is_equal_to(&String::from("mint"));

        let query_res = chain.query::<
                cw20_base::msg::QueryMsg,
                Response
            >(&cw20_base::msg::QueryMsg::Balance {
                address: recipient.to_string()
            }, &contract_address).unwrap();

        asserting("that query passed on correctly")
            .that(&query_res.attributes[1].value)
            .is_equal_to(&String::from("0"));

        // is migration not yet implemented?
        // let migration_res = chain.migrate(&MigrateMsg{}, 1, &contract_address);
        // println!("{:#?}", migration_res);
    }
}