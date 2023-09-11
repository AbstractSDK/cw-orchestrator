# Daemon

Daemon is an execution environment for interacting with live chains. This allows you to test your deployment on a local chain instance or deploy your contracts on mainnet. `Cw-orchestrator Daemon` provides a wide range of tools to interact with live chains. We describe those tools in depth in this page.

## Quick Start

The `daemon` integration is really straightforward to integrate for developers. Creating a daemon instance goes along the lines of : 

```rust,ignore
    use cw_orch::prelude::*;
    use tokio::runtime::Runtime;
{{#include ../../../cw-orch/examples/daemon_test.rs:daemon_creation}}
```
The `chain` parameter allows you to specify which chain you want to interact with. The chains that are officially supported can be found in the `cw_orch::daemon::networks` namespace. But any chain from the [`ibc-chain-registry`](https://crates.io/crates/ibc-chain-registry) can be used as a chain argument to the `DaemonBuilder` object.

The `handle` parameter is a *tokio runtime handle*. 

<details>
  <summary>Develop if you <strong>don't know</strong> what that means</summary>
  
  
You don't need to know all the intricacies of <a href="https://rust-lang.github.io/async-book/">tokio and rust-async</a> to use daemons. If you don't have time to learn more, you can simply use the snippet above and not touch the runtime creation. However for more advanced `daemon` usage and also for your culture, don't hesitate to learn about those wonderful processes and their management.
      
</details>

<details>
  <summary>Develop if you <strong>know</strong> what that means</summary>

This handler is used because all the front-facing daemon methods are synchronous. However everything that's happening in the background is asynchronous. This handle is used exclusively to await asynchronous function : 
```rust,ignore
    runtime.block_on(...)

```
Because creating runtimes is a costly process, we leave the handler creation and management to the user.
      
</details>


This simple script actually hides another parameter which is the `LOCAL_MNEMONIC` environment variable. This variable is used when interacting with local chains. See the part dedicated to [Environment Vars](../single_contract/env-variable.md) for more details.


> **_NOTE:_** When using `daemon`, you are interacting directly with a live chain. The program won't ask you for your permission at each step of the script. We advise you to test **ALL** your deployments on test chain before deploying to mainnet.

## Interacting with contracts

You can then use the resulting `Daemon` variable to interact with your [contracts](../single_contract/index.md): 

```rust,ignore
{{#include ../../../cw-orch/examples/daemon_test.rs:daemon_usage}}
```

All contract operations will return an object of type `cw_orch::prelude::CosmTxResponse`. This represents a successful transaction. Using the txhash of the tx, you can also inspect the operations on a chain explorer.

> **_ADVICE:_** Add `RUST_LOG=INFO` to your environment and use the `env_logger::init()` initializer to get detailed information about your script execution. Cw-orchestrator provides enhanced logging tools for following the deployment and potentially pick up where you left off.


> This environment needs wasm artifacts to deploy the contracts to the chains. Don't forget to compile all your wasm before deploying your contracts !

> If you are using the customizable Interface Macro, you will need to have implemented the `wasm` function for interacting the the `Daemon` environment. 

## State management

In order to manage your contract deployments cw-orchestrator saves the contract addresses and code ids for each network you're interacting with in a JSON formatted state file. This state file represents all your past. You can customize the path to this state file using the `STATE_FILE` [env variable](../single_contract/env-variable.md). 

When calling the `upload` function on a contract, if the tx is successful, the daemon will get the uploaded code_id and save it to file, like so : 

```json
{
  "juno": {
    "juno-1": {
      "code_ids": {
        "counter_contract": 1356,
      },     
    }
  }
}

```

In this example: 
- `counter_contract`  corresponds to the `contract_id`variable (the one that you can set in the [contract interface constructor](../single_contract/interfaces.html#constructor)).
- `default` correspond to the deployment namespace. This can be set when building the daemon object in order to separate multiple deployments. You could have you `juno-counter-v1` deployment`


When calling the `instantiate` function, if the tx is successful, the daemon will get the contract address and save it to file, like so :
```json 
{
  "juno": {
    "juno-1": {
      "code_ids": {
        "counter_contract": 1356,
      },
      "default": {
        "counter_contract": "juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d"
      }
    }
  }
}
```

the default keyword stand here for the name of the contract deployment. That way you can handle multiple contract deployment 

## Additional tools

The `Daemon` environment provides a bunch of tools for you to interact in a much easier way with the blockchain. Here is a non-exhaustive list : 
- Get the sender address easily (from the registered mnemonic)
  ```rust,ignore
    let sender_addr = daemon.sender()
  ```
- Send more transaction types 
   ```rust,ignore
    let wallet = daemon.wallet();
    // For sending native funds to a wallet
    wallet.bank_send("<address-of-my-sister>", coins(345, "ujunox")).await?; 
    // For executing any type of message
    wallet.commit_tx(vec![cosmrs::staking::MsgBeginRedelegate {
      /// Delegator's address.
      pub delegator_address: AccountId::from_str("<my-address">)?,

      /// Source validator's address.
      pub validator_src_address: AccountId::from_str("<my-least-favorite-validator">)?,

      /// Destination validator's address.
      pub validator_dst_address: AccountId::from_str("<my-favorite-validator">)?,

      /// Amount to UnDelegate
      pub amount: coin(100_000_000_000_000, "ujuno"),
  }]).await?; 
  ```
- Simulate txs without sending them (no examples here because this is more difficult to use)

## Queries

The daemon object can also be used to execute queries to the chains we are interacting with. 
This opens up a lot more applications to cw-orchestrator as this tools can also be used to manage off-chain applications.

Querying the chain for data using a daemon looks like : 
```rust,ignore
{{#include ../../../cw-orch/examples/queries/bank_query.rs:daemon_balance_query}}
```

> **_NOTE:_** This is not usable in contracts. This is only for off-chain querying !!

For more information and queries, [visit the daemon querier implementations directly](https://docs.rs/crate/cw-orch/latest/source/src/daemon/queriers.rs)