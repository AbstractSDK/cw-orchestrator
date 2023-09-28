# Fork Testing

> **NOTE**: This feature is not publicly available yet. If you want preview access to this testing feature, please reach out via our [website](https://abstract.money/).


Cw-orchestrator supports testing in a forked environment. With this feature, you can execute the application you are developing on a blockchain environment without having to spin up a node yourself and create a fork locally. All the code and application you will be running during the test will be rust code and the storage needed to execute is minimal as only the necessary data is downloaded from actual blockchain nodes. 


## Brief Overview

We leverage the beautiful [`cw-multi-test`](https://github.com/CosmWasm/cw-multi-test/) package, created and maintained by Cosmwasm and added a few functionalities that allows your to execute your code just as if you were interacting with an actual on-chain node, but locally, and without any on-chain funds necessary.

### Setup
Setting up the environment is really easy and only requires feeding a `ChainData` object to the struct constructor : 

```rust
use cw_orch::networks::JUNO_1;
use cw_orch::ForkMock;

let app = ForkMock::new(JUNO_1)?;
```

With this, you are ready to upload, instantiate, migrate, interact with on-chain contracts...

### Execution Flow

This execution environment has a mixed behavior.

#### Execution 
    
The blockchain modules logic is implemented in rust. Every module that is not standard has to be present and implemented in rust to be available. Because we are based on `cw-multi-test`, our application is compatible with cw-multi-test modules. For now :

- The Wasm module is implemented locally via the `cosmwasm-vm` package that handles wasm execution.
- The Bank module is implemented by `cw-multi-test` and available in this environment.
- The Staking module is not fully implemented because distant storage is more difficult to query. It's the next module we wish to implement. 

#### Storage

This part is actually at the center of the innovation provided by this package. 
- When reading blockchain storage, the testing environment will execute in order : 
  1. Check local storage for data availability. If some data was stored locally, use that value
  2. If not, check the registered blockchain node for data availability and error if it's not available. 

- When writing value to storage, nothing changes. 

Let's take an example for clarity. Say I want to deposit some funds into Anchor Protocol. Here are the steps that a user would have to go through and how they are executed inside the environment.


1. The user needs funds to interact with the protocol. A `fork-helper` allows to increase the balance of an address.
   ```rust
    let sender = "terra1..."; // Sender address. Can also be an actual account with funds.
    // We add some funds specific for our application
    app.init_modules(|router, _, storage| {
        router.bank
            .init_balance(storage, &Addr::unchecked(sender), coins(10_000_000, "uusd"))
    })?;
   ```
2. The user calls the following `ExecuteMsg` on the actual mainnet Anchor Moneymarket Contract :
    ```rust
    let market = "terra1..."; // Actual contract address of the Anchor deployment.
    app.execute_contract(
            Addr::unchecked(sender),
            Addr::unchecked(market),
            &ExecuteMsg::DepositStable {},
            &coins(10_000, "uusd"),
        )?;
    ```
1. During message execution, the contract code will be queried from the chain and executed locally. During contract execution, all the necessary storage values will also be retrieved as the programs needs them. No local storage is used until something is written to it [^storage_cache].
2. Even in the case of multiple chained contract calls, storage is modified accordingly and usable by contracts. 
3. After message execution, queries and states are modified according the contract execution. After depositing, it is now possible to query your stake or even to withdraw your funds : 
    ```rust
    let a_currency = "terra1..."; // The contract address for the staking receipt

    /// This should give a non-zero value, even if no change occurred on the actual mainnet state
    let response: BalanceResponse = app
        .wrap()
        .query_wasm_smart(
            a_currency,
            &Cw20QueryMsg::Balance {
                address: sender.to_string(),
            },
        )?;

    /// We can get our funds back, no problem
    app.execute_contract(
        Addr::unchecked(sender),
        Addr::unchecked(market),
        &ExecuteMsg::RedeemAllStable {},
        &[],
    )?;
    ```

## Usage

You use this fork environment as you would use the `Mock` environment, with a few subtle changes : 
1. You can't use human readable addresses, because this environment uses actual APIs and needs to be able to verify addresses. In order to generate a new address, you can use : 

    ```rust
    let new_sender = fork.init_account();
    ```
 2. The environment doesn't allow (yet) contracts to be defined using its functions. A contract in this environment is executed through its compiled wasm. When testing with this environment, make sure that you compile your project before running the tests. We are aware that this is very easy on the user and are working on being able to accept both structure that implement the `Contract` trait AND wasm files.



[^storage_cache]In the future, we might leverage a local storage cache to avoid querying distant RPCs too much (for more speed and less data consumption).