//! Live mock is a mock that uses a live chain to query for data.
//! It can be used to do chain-backed unit-testing. It can't be used for state-changing operations.

use crate::queriers::Bank;
use crate::queriers::CosmWasm;
use crate::queriers::Staking;
use crate::RUNTIME;
use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::Addr;
use cosmwasm_std::AllBalanceResponse;
use cosmwasm_std::BalanceResponse;
use cosmwasm_std::BankQuery;
use cosmwasm_std::Binary;
use cosmwasm_std::Delegation;
use cosmwasm_std::Empty;
use cosmwasm_std::StakingQuery;
use cosmwasm_std::{
    from_json, to_json_binary, Coin, ContractResult, OwnedDeps, Querier, QuerierResult,
    QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};
use cosmwasm_std::{AllDelegationsResponse, BondedDenomResponse};
use cw_orch_core::environment::BankQuerier;
use cw_orch_core::environment::WasmQuerier;
use cw_orch_networks::ChainInfoOwned;
use std::marker::PhantomData;
use std::str::FromStr;
use tonic::transport::Channel;

use crate::channel::GrpcChannel;

fn to_cosmwasm_coin(c: cosmrs::proto::cosmos::base::v1beta1::Coin) -> Coin {
    Coin {
        amount: Uint128::from_str(&c.amount).unwrap(),
        denom: c.denom,
    }
}

const QUERIER_ERROR: &str =
    "Only Bank balances and Wasm (raw + smart) and Some staking queries are covered for now";

/// mock_dependencies is a drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// this uses our CustomQuerier.
pub fn mock_dependencies(
    chain_info: ChainInfoOwned,
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier = WasmMockQuerier::new(chain_info);

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

/// Querier struct that fetches queries on-chain directly
pub struct WasmMockQuerier {
    channel: Channel,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<Empty> = match from_json(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    /// Function used to handle a query and customize the query behavior
    /// This implements some queries by querying an actual node for the responses
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        let handle = RUNTIME.handle();
        match &request {
            QueryRequest::Wasm(x) => {
                let querier = CosmWasm {
                    channel: self.channel.clone(),
                    rt_handle: Some(handle.clone()),
                };
                match x {
                    WasmQuery::Smart { contract_addr, msg } => {
                        // We forward the request to the cosmwasm querier

                        let query_result: Result<Binary, _> = handle
                            .block_on(
                                querier._contract_state(contract_addr.to_string(), msg.to_vec()),
                            )
                            .map(|query_result| query_result.into());
                        SystemResult::Ok(ContractResult::from(query_result))
                    }
                    WasmQuery::Raw { contract_addr, key } => {
                        // We forward the request to the cosmwasm querie
                        let query_result = querier
                            .raw_query(contract_addr.to_string(), key.to_vec())
                            .map(|query_result| query_result.into());

                        SystemResult::Ok(ContractResult::from(query_result))
                    }
                    _ => SystemResult::Err(SystemError::InvalidRequest {
                        error: QUERIER_ERROR.to_string(),
                        request: to_json_binary(&request).unwrap(),
                    }),
                }
            }
            QueryRequest::Bank(x) => {
                let querier = Bank {
                    channel: self.channel.clone(),
                    rt_handle: Some(handle.clone()),
                };
                match x {
                    BankQuery::Balance { address, denom } => {
                        let query_result =
                            querier.balance(address, Some(denom.clone())).map(|result| {
                                to_json_binary(&BalanceResponse {
                                    amount: result[0].clone(),
                                })
                                .unwrap()
                            });
                        SystemResult::Ok(ContractResult::from(query_result))
                    }
                    BankQuery::AllBalances { address } => {
                        let query_result = querier
                            .balance(address, None)
                            .map(|result| AllBalanceResponse { amount: result })
                            .map(|query_result| to_json_binary(&query_result))
                            .unwrap();
                        SystemResult::Ok(ContractResult::from(query_result))
                    }
                    _ => SystemResult::Err(SystemError::InvalidRequest {
                        error: QUERIER_ERROR.to_string(),
                        request: to_json_binary(&request).unwrap(),
                    }),
                }
            }
            QueryRequest::Staking(x) => {
                let querier = Staking::new_async(self.channel.clone());
                match x {
                    StakingQuery::BondedDenom {} => {
                        let query_result = handle
                            .block_on(querier._params())
                            .map(|result| BondedDenomResponse {
                                denom: result.params.unwrap().bond_denom,
                            })
                            .map(|query_result| to_json_binary(&query_result))
                            .unwrap();
                        SystemResult::Ok(ContractResult::from(query_result))
                    }
                    // This query is not perfect. I guess that on_chain you should be able to get ALL delegations and not a paginated result
                    // TODO, do better here
                    StakingQuery::AllDelegations { delegator } => {
                        let query_result = handle
                            .block_on(querier._delegator_delegations(delegator, None))
                            .map(|result| AllDelegationsResponse {
                                delegations: result
                                    .delegation_responses
                                    .into_iter()
                                    .filter_map(|delegation| {
                                        delegation.delegation.map(|d| Delegation {
                                            delegator: Addr::unchecked(d.delegator_address),
                                            validator: d.validator_address,
                                            amount: to_cosmwasm_coin(delegation.balance.unwrap()),
                                        })
                                    })
                                    .collect(),
                            })
                            .map(|query_result| to_json_binary(&query_result))
                            .unwrap();
                        SystemResult::Ok(ContractResult::from(query_result))
                    }
                    _ => todo!(),
                }
            }
            _ => SystemResult::Err(SystemError::InvalidRequest {
                error: QUERIER_ERROR.to_string(),
                request: to_json_binary(&request).unwrap(),
            }),
        }
    }
}

impl WasmMockQuerier {
    /// Creates a querier from chain information
    pub fn new(chain: ChainInfoOwned) -> Self {
        let channel = RUNTIME
            .block_on(GrpcChannel::connect(
                &chain.grpc_urls,
                chain.chain_id.as_str(),
            ))
            .unwrap();

        WasmMockQuerier { channel }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::networks::JUNO_1;

    use super::mock_dependencies;

    #[test]
    fn bank_balance_querier() -> Result<(), anyhow::Error> {
        let address = "juno1rkhrfuq7k2k68k0hctrmv8efyxul6tgn8hny6y";

        let deps = mock_dependencies(JUNO_1.into());
        let deps_ref = deps.as_ref();
        let _response: BalanceResponse =
            deps_ref
                .querier
                .query(&QueryRequest::Bank(BankQuery::Balance {
                    address: address.to_string(),
                    denom: "ujuno".to_string(),
                }))?;
        // We can't really test that response, but it has to unwrap at least !

        Ok(())
    }

    #[test]
    fn bank_all_balances_querier() -> Result<(), anyhow::Error> {
        let address = "juno1rkhrfuq7k2k68k0hctrmv8efyxul6tgn8hny6y";

        let deps = mock_dependencies(JUNO_1.into());
        let deps_ref = deps.as_ref();
        let _response: AllBalanceResponse =
            deps_ref
                .querier
                .query(&QueryRequest::Bank(BankQuery::AllBalances {
                    address: address.to_string(),
                }))?;
        // We can't really test that response, but it has to unwrap at least !
        Ok(())
    }
}
