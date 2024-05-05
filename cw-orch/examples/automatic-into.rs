use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_orch::prelude::*;
use msg::{
    execute::{BaseExecMsg, BaseExecMsgFns as _, MintingExecMsg, MintingExecMsgFns as _},
    query::{
        BalanceResponse, BaseQueryMsg, BaseQueryMsgFns as _, MinterResponse, MintingQueryMsg,
        MintingQueryMsgFns as _,
    },
};

#[cw_orch::interface(Empty, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw20;

#[cw_orch::interface(Empty, MintingExecMsg, Empty, Empty)]
pub struct Cw20Minter;

#[cw_orch::interface(Empty, BaseExecMsg, Empty, Empty)]
pub struct Cw20Base;

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn minter_execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: MintingExecMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn base_execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: BaseExecMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Minting(minting) => match minting {
            MintingQueryMsg::Minter {} => to_json_binary(&MinterResponse {
                minter: "minter".to_string(),
            }),
        },
        QueryMsg::Base(base_msg) => match base_msg {
            BaseQueryMsg::Balance { address: _ } => to_json_binary(&BalanceResponse {
                balance: 167u128.into(),
            }),
        },
    }
}

pub fn migrate(_deps: DepsMut, _env: Env, msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}

impl<Chain> Uploadable for Cw20<Chain> {
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(ContractWrapper::new_with_empty(execute, instantiate, query).with_migrate(migrate))
    }
}

impl<Chain> Uploadable for Cw20Minter<Chain> {
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(minter_execute, instantiate, query)
                .with_migrate(migrate),
        )
    }
}

impl<Chain> Uploadable for Cw20Base<Chain> {
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(base_execute, instantiate, query).with_migrate(migrate),
        )
    }
}

pub fn main() -> anyhow::Result<()> {
    let mock = MockBech32::new("mock");

    let contract = Cw20::new("cw20", mock.clone());
    contract.upload()?;
    contract.instantiate(&Empty {}, None, None)?;

    contract.mint(150_100u128.into(), "nicoco".to_string())?;
    contract.send(
        150_100u128.into(),
        "nicoco".to_string(),
        Binary::from_base64("cXNk")?,
    )?;
    let minter_response = contract.minter()?;
    let balance = contract.balance("nicoco".to_string())?;
    assert_eq!(minter_response.minter, "minter");
    assert_eq!(balance.balance.u128(), 167);

    let contract = Cw20Minter::new("cw20_minter", mock.clone());
    contract.upload()?;
    contract.instantiate(&Empty {}, None, None)?;
    contract.mint(150_100u128.into(), "nicoco".to_string())?;

    let contract = Cw20Base::new("cw20_base", mock.clone());
    contract.upload()?;
    contract.instantiate(&Empty {}, None, None)?;
    contract.send(
        150_100u128.into(),
        "nicoco".to_string(),
        Binary::from_base64("cXNk")?,
    )?;

    Ok(())
}

#[cw_serde]
pub enum ExecuteMsg {
    Minting(MintingExecMsg),
    Base(BaseExecMsg),
}

impl From<MintingExecMsg> for ExecuteMsg {
    fn from(value: MintingExecMsg) -> Self {
        Self::Minting(value)
    }
}

impl From<BaseExecMsg> for ExecuteMsg {
    fn from(value: BaseExecMsg) -> Self {
        Self::Base(value)
    }
}

#[cw_serde]
pub enum QueryMsg {
    Minting(MintingQueryMsg),
    Base(BaseQueryMsg),
}
impl From<MintingQueryMsg> for QueryMsg {
    fn from(value: MintingQueryMsg) -> Self {
        Self::Minting(value)
    }
}

impl From<BaseQueryMsg> for QueryMsg {
    fn from(value: BaseQueryMsg) -> Self {
        Self::Base(value)
    }
}

mod msg {
    pub mod execute {

        use cosmwasm_schema::cw_serde;
        use cosmwasm_std::{Binary, Uint128};

        #[cw_serde]
        #[derive(cw_orch::ExecuteFns)]
        pub enum MintingExecMsg {
            Mint { recipient: String, amount: Uint128 },
        }

        #[cw_serde]
        #[derive(cw_orch::ExecuteFns)]
        pub enum BaseExecMsg {
            Send {
                contract: String,
                amount: Uint128,
                msg: Binary,
            },
        }
    }

    pub mod query {

        use cosmwasm_schema::{cw_serde, QueryResponses};
        use cosmwasm_std::Uint128;
        use cw_orch::prelude::*;

        #[cw_serde]
        #[derive(QueryResponses, cw_orch::QueryFns)]
        pub enum MintingQueryMsg {
            #[returns(MinterResponse)]
            Minter {},
        }

        #[cw_serde]
        #[derive(QueryResponses, cw_orch::QueryFns)]
        pub enum BaseQueryMsg {
            #[returns(BalanceResponse)]
            Balance { address: String },
        }
        #[cw_serde]
        pub struct MinterResponse {
            pub minter: String,
        }
        #[cw_serde]
        pub struct BalanceResponse {
            pub balance: Uint128,
        }
    }
}
