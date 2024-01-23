# Unit Tests

Cw-orchestrator provides an additional tool to help you with unit tests with context. The <a href="https://docs.rs/cw-orch/latest/cw_orch/daemon/live_mock/struct.WasmMockQuerier.html" target="_blank">`WasmMockQuerier`</a> object allows you to leverage `cw-orch` querying abilities to unit-test your contracts.

## Capabilities

The `WasmMockQuerier` (and associated `mock_dependencies` helper function) allow developers to unit-test their contract against on-chain logic. This allows for a mix between unit and integration tests for application that need to rely on on-chain data. This structure works very similarly to `cosmwasm_std::testing::mock_dependencies` but instead of querying for local scripts or data, it queries the information from an actual running blockchain.

Today, the following modules are supported by this querier:

- Wasm
- Bank
- Staking (support has not been finalized as of yet)

## Example

Let's imagine you want to build a lending aggregator. In this kind of application, you want to query the balance in staking tokens that your client has. In order to do that, you may want to use the following syntax inside your contract:

```rust,ignore
fn query_balance(deps: Deps, client: Addr, lending_contract: Addr) -> Result<BalanceResponse, ContractError>{
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart{
        contract_addr: lending_contract,
        query: Binary::from({
            StakingBalance{
                ...
            }
        })
    }))?;
}
```

In order to unit-test that logic, you may want to test against actual on-chain data. The following code_snippet will allow you to test this piece of code against actual on-chain code.

```rust,ignore
#[test]
fn balance_is_available() -> anyhow::Result<()>{
    let deps = cw_orch::prelude::live_mock::mock_dependencies(JUNO_1);

    // assert the query is successful
    let balance_response = query_balance(deps.as_ref(), Addr::unchecked("juno1..."), Addr::unchecked("juno1..")).unwrap();
}
```

Obviously, you need to specify existing on-chain addresses to be able to use the `cw_orch` `mock_dependencies` function. Those dependencies have a very similar behavior to `cosmwasm_std::testing::mock_dependencies`, expect the query response is fetched from on-chain information.
