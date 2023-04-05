/*
    Mock tests
*/
#[cfg(test)]
mod general {
    use cosmwasm_std::{Addr, Coin, Response, to_binary, DepsMut, Env, MessageInfo, StdResult, Deps, Binary, Uint128};

    use speculoos::prelude::*;

    use cw_multi_test::ContractWrapper;

    use crate::{
        mock::core::*,
        TxHandler,
        ContractCodeReference, ChainState, StateInterface
    };

    const SENDER: &str = "cosmos123";
    const BALANCE_ADDR: &str = "cosmos456";

    fn execute(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: cw20::Cw20ExecuteMsg,
    ) -> Result<Response, cw20_base::ContractError>
    {
        match msg {
            cw20::Cw20ExecuteMsg::Transfer { recipient, amount } => todo!(),
            cw20::Cw20ExecuteMsg::Burn { amount } => todo!(),
            cw20::Cw20ExecuteMsg::Send { contract, amount, msg } => todo!(),
            cw20::Cw20ExecuteMsg::IncreaseAllowance { spender, amount, expires } => todo!(),
            cw20::Cw20ExecuteMsg::DecreaseAllowance { spender, amount, expires } => todo!(),
            cw20::Cw20ExecuteMsg::TransferFrom { owner, recipient, amount } => todo!(),
            cw20::Cw20ExecuteMsg::SendFrom { owner, contract, amount, msg } => todo!(),
            cw20::Cw20ExecuteMsg::BurnFrom { owner, amount } => todo!(),
            cw20::Cw20ExecuteMsg::Mint { recipient, amount } => {
                Ok(
                    Response::default()
                        .add_attribute("action", "mint")
                        .add_attribute("recipient", recipient)
                        .add_attribute("amount", amount)
                )
            },
            cw20::Cw20ExecuteMsg::UpdateMinter { new_minter } => todo!(),
            cw20::Cw20ExecuteMsg::UpdateMarketing { project, description, marketing } => todo!(),
            cw20::Cw20ExecuteMsg::UploadLogo(_) => todo!(),
        }
    }

    fn instantiate(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: cw20_base::msg::InstantiateMsg,
    ) -> StdResult<Response> {
        Ok(Response::default())
    }

    fn query(
        _deps: Deps,
        _env: Env,
        _msg: cw20_base::msg::QueryMsg
    ) -> StdResult<Binary> {
        Ok(
            to_binary(
                &Response::<cw20_base::msg::QueryMsg>::default()
            ).unwrap()
        )
    }

    #[test]
    fn mock() {
        let sender = &Addr::unchecked(SENDER);
        let mock = instantiate_default_mock_env(sender).unwrap();
        let chain = mock.1;

        let recipient = &Addr::unchecked(BALANCE_ADDR);
        let amount = 1000000u128;
        let denom = "uosmo";

        chain.set_balance(recipient, vec![Coin::new(amount, denom)]).unwrap();
        let balance = chain.query_balance(recipient, denom).unwrap();

        asserting("address balance is the correct").that(&amount).is_equal_to(&balance.into());
        asserting("sender is correct").that(sender).is_equal_to(chain.sender());

        let mut contract_source: ContractCodeReference = ContractCodeReference::default();

        contract_source.contract_endpoints = Some(Box::new(
            ContractWrapper::new(execute, instantiate, query)
        ));

        let res = chain.upload(&mut contract_source).unwrap();
        asserting("contract initialized properly")
            .that(&res.events[0].attributes[0].value)
            .is_equal_to(&String::from("1"));

        let init_msg = cw20_base::msg::InstantiateMsg {
            name: String::from("Token"),
            symbol: String::from("TOK"),
            decimals: 6u8,
            initial_balances: vec![],
            mint: None,
            marketing: None,
        };
        let init_res = chain.instantiate(1, &init_msg, None, None, &[]).unwrap();

        // println!("{:#?}", init_res.events[0].attributes[0].value);

        let contract_address = Addr::unchecked(&init_res.events[0].attributes[0].value);
        // let contract_address = chain.state().get_address(&"1").unwrap();

        // println!("{:#?}", contract_address);

        let res = chain.execute(
            &cw20_base::msg::ExecuteMsg::Mint { recipient: recipient.to_string(), amount: Uint128::from(100u128) },
            &[],
            &contract_address);

        println!("{:#?}", res);

        // chain.query(query_msg, contract_address)

        assert!(false)
    }
}