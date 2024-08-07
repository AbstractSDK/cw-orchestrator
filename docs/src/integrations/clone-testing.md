# Clone Testing

Cw-orchestrator supports testing in a forked environment. With this feature, you can execute the application you are developing on a blockchain environment without having to spin up a node yourself and create a fork locally. This means that it will simulate having the same state as mainnet, but it will run locally inside your Rust code. All the code and application you will be running during the test will be rust code and the storage needed to execute is minimal as only the necessary data is downloaded from actual blockchain nodes.

## Brief Overview

We leverage the beautiful <a href="https://github.com/CosmWasm/cw-multi-test/" target="_blank">`cw-multi-test`</a> package, created and maintained by Cosmwasm and added a few functionalities that allow you to execute your code just as if you were interacting with an actual on-chain node, but locally, and without any on-chain funds necessary.

### Setup

Before using clone testing, import the following crate <a href="https://crates.io/crates/cw-orch-clone-testing" target="_blank">`cw-orch-clone-testing`</a>

Setting up the environment is really easy and only requires feeding a `ChainData` object to the struct constructor:

```rust,ignore
use cw_orch::networks::JUNO_1;
use cw_orch_clone_testing::CloneTesting;

let app = CloneTesting::new(JUNO_1)?;
```

With this, you are ready to upload, instantiate, migrate and interact with on-chain contracts...

You can find an <a href="https://github.com/AbstractSDK/cw-orchestrator/tree/main/packages/clone-testing/tests/clone-testing.rs" target="_blank">advanced example</a> in the cw-orch repository.

### Execution Flow

This execution environment has a mixed behavior.

#### Execution

The blockchain modules logic is implemented in rust. Every module that is not standard has to be present and implemented in rust to be available. Because we are based on `cw-multi-test`, our application is compatible with cw-multi-test modules. For now:

- The Wasm module is implemented locally via a mixed approach that allows execution of:
  - Rust Code via `cw-multi-test`
  - Wasm binaries via the `cosmwasm-vm` package.
  
- The Bank module is implemented by `cw-multi-test` and available in this environment.
- The Staking module is not fully implemented because distant storage is more difficult to query. It's the next module we wish to implement.

#### Storage

This part is actually at the center of the innovation provided by this package.

- When reading blockchain storage, the testing environment will execute in order:
  1. Check local storage for data availability. If some data was stored locally, use that value
  2. If not, check the registered blockchain node for data availability and error if it's not available.

- When writing value to storage, nothing changes.

Let's take an example for clarity. Say I want to deposit some funds into Anchor Protocol. Here are the steps that a user would have to go through and how they are executed inside the environment.

1. The user needs funds to interact with the protocol. A `fork-helper` allows to increase the balance of an address.

   ```rust,ignore
    // Sender address. Can also be an actual account with funds.
    // Could also be app.sender() for creating an address automatically.
    let sender = "terra1..."; 
    let sender_addr = Addr::unchecked(sender);
    app.set_sender(&sender_addr);
    // We add some funds specific for our application
    app.set_balance(&sender_addr, coins(10_000_000, "uusd"))?;
   ```

2. The user calls the following `ExecuteMsg` on the actual mainnet Anchor Moneymarket Contract:

    ```rust,ignore
    let market_addr = Addr::unchecked("terra1..."); // Actual contract address of the Anchor deployment.
    let market = AnchorMarket::new("anchor:money-market", app.clone());
    market.set_address(&market_addr);
    market.deposit_stable(&coins(10_000, "uusd"))?;
    ```

3. During the whole message execution, when storage is queried, if it doesn't exist locally it will be queried from the chain. This is true for storage during contract execution but this is also true for querying the actual Wasm Code when executing/querying a contract. No local storage is used until something is written to it [^storage-cache].
4. Even in the case of multiple chained contract calls, storage is modified accordingly and usable by contracts.
5. After message execution, queries and states are modified according to the contract execution. After depositing, it is now possible to query your stake or even to withdraw your funds:

    ```rust,ignore
    let a_currency = "terra1..."; // The contract address for the staking receipt

    /// This should give a non-zero value, even if no change occurred on the actual mainnet state
    let response: BalanceResponse = app
        .query(&Cw20QueryMsg::Balance {
                address: sender.to_string(),
            },
            &Addr::unchecked(a_currency),
        )?;

    /// We can get our funds back, no problem, the state changes get propagated as well locally
    market.redeem_all_stable()?;
    ```

## Usage

You use this fork environment as you would use the `Mock` environment, with a few subtle changes:

1. You can't use human readable addresses, because this environment uses actual APIs and needs to be able to verify addresses. When creating the Mock environment, a sender gets created along and attach automatically to the `CloneTesting` instance. If you need additional addresses, you can use:

    ```rust,ignore
    let new_sender: Addr = fork.init_account();
    ```

2. The environment allows for using contracts defined using its functions (just like the [`Mock`](./cw-multi-test.md) can) **AND** compiled WASM contracts. By default, the Rust `wrapper` method is used to upload the contract in the environment. The following code will use this wrapper. This makes it really easy and fast to iterate on your contracts (for migrating, debugging...):

    ```rust,ignore
    use cw_orch::prelude::*;
{{#include ../../../packages/clone-testing/tests/wasm-upload.rs:clone_testing_setup}}
{{#include ../../../packages/clone-testing/tests/wasm-upload.rs:counter_contract_setup}}
{{#include ../../../packages/clone-testing/tests/wasm-upload.rs:upload}}
    ```

    If you prefer using Wasm compiled smart contracts, use the following snippet: 

    ```rust,ignore
    use cw_orch::prelude::*;
{{#include ../../../packages/clone-testing/tests/wasm-upload.rs:clone_testing_setup}}
{{#include ../../../packages/clone-testing/tests/wasm-upload.rs:counter_contract_setup}}
{{#include ../../../packages/clone-testing/tests/wasm-upload.rs:upload_wasm}}
    ```

[^storage-cache]: In the future, we might leverage a local storage cache to avoid querying distant RPCs too much (for more speed and less data consumption).
