/*
    Mock tests
*/
#[cfg(test)]
mod general {
    use cosmwasm_std::{Addr, Coin, Response, to_binary, DepsMut, Env, MessageInfo, StdResult, Deps, Binary};

    use speculoos::prelude::*;

    use cw_multi_test::ContractWrapper;

    use crate::{
        mock::core::*,
        TxHandler,
        ContractCodeReference
    };

    const SENDER: &str = "cosmos123";
    const BALANCE_ADDR: &str = "cosmos456";

    fn execute(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: cw20::Cw20ExecuteMsg,
    ) -> Result<Response, cw20_base::ContractError>
    {
        Ok(Response::default())
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

        let res = chain.upload(&mut contract_source);
        match res {
            Ok(r) => println!("{:#?}", r),
            Err(e) => println!("{:#?}", e),
        }

        assert!(false)
    }
}