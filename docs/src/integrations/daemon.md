# Daemon

`Daemon` is a CosmWasm execution environment for interacting with CosmosSDK chains. The `Daemon` allows you to deploy/migrate/configure your contracts on main and testnets as well as locally running chain instances. Furthermore it provides a wide range of tools to interact with the chains. We describe those tools in depth in this page.

## Quick Start

Interacting with the `daemon` is really straightforward. Creating a daemon instance is shown below:

```rust,ignore
    use cw_orch::prelude::*;
    use tokio::runtime::Runtime;
{{#include ../../../cw-orch/examples/local_daemon.rs:daemon_creation}}
```

- The `chain` parameter allows you to specify which chain you want to interact with. The chains that are officially supported can be found in the `cw_orch::daemon::networks` module.
  You can also add additional chains yourself by simply defining a variable of type <a href="https://docs.rs/cw-orch/latest/cw_orch/daemon/struct.ChainInfo.html" target="_blank">`ChainInfo`</a> and using it in your script. Don't hesitate to open a PR on the <a href="https://github.com/AbstractSDK/cw-orchestrator" target="_blank">cw-orchestrator repo</a>, if you would like us to include a chain by default. The variables needed for creating the variable can be found in the documentation of the chain you want to connect to or in the <a href="https://cosmos.directory" target="_blank">Cosmos Directory</a>.

This simple script actually hides another parameter which is the `LOCAL_MNEMONIC` environment variable. This variable is used when interacting with local chains. See the part dedicated to [Environment Vars](../contracts/env-variable.md) for more details.

> **_NOTE:_** When using `daemon`, you are interacting directly with a live chain. The program won't ask you for your permission at each step of the script. We advise you to test **ALL** your deployments on test chain before deploying to mainnet.
>
> Under the hood, the `DaemonBuilder` struct creates a `tokio::Runtime`. Be careful because this builder is not usable in an `async` function. In such function, you can use <a href="https://docs.rs/cw-orch/latest/cw_orch/daemon/struct.DaemonAsync.html" target="_blank">`DaemonAsync`</a>

<div class="warning">

When using multiple Daemons with the same state file, you should re-use a single Daemon State to avoid conflicts and panics: 

```rust,ignore
let daemon1 = Daemon::builder()
  .chain(OSMOSIS_1)
  .build()?;
// If you don't use the `state` method here, this will fail with:
// State file <file-name> already locked, use another state file, clone daemon which holds the lock, or use `state` method of Builder
let daemon2 = Daemon::builder()
  .chain(JUNO_1)
  .state(daemon1.state())
  .build()?;
```


</div>



## Interacting with contracts

You can then use the resulting `Daemon` variable to interact with your [contracts](../contracts/index.md):

```rust,ignore
{{#include ../../../cw-orch/examples/local_daemon.rs:daemon_usage}}
```

All contract operations will return an object of type `cw_orch::prelude::CosmTxResponse`. This represents a successful transaction. Using the txhash of the tx, you can also inspect the operations on a chain explorer.

> **_ADVICE:_** Add `RUST_LOG=INFO` to your environment and use the `env_logger::init()` initializer to get detailed information about your script execution. Cw-orchestrator provides enhanced logging tools for following the deployment and potentially pick up where you left off.
> This environment needs wasm artifacts to deploy the contracts to the chains. Don't forget to compile all your wasms before deploying your contracts !

## State management

In order to manage your contract deployments cw-orchestrator saves the contract addresses and code ids for each network you're interacting with in a JSON formatted state file. This state file represents all your past. You can customize the path to this state file using the `STATE_FILE` [env variable](../contracts/env-variable.md).

When calling the `upload` function on a contract, if the tx is successful, the daemon will get the uploaded code_id and save it to file, like so:

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

In this example: `counter_contract`  corresponds to the `contract_id`variable (the one that you can set in the [contract interface constructor](../contracts/interfaces.html#constructor)).

When calling the `instantiate` function, if the tx is successful, the daemon will get the contract address and save it to file, like so:

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

In this example, the `default` keyword corresponds to the deployment namespace. This can be set when building the daemon object (using the `DaemonBuilder::deployment_id` method) in order to separate multiple deployments. For instance for a DEX (decentralized exchange), you can have a single code-id but multiple pool addresses for all your liquidity pools. You would have a `juno-usdc` and a `usdt-usdc` deployment, sharing the same code-ids but different contract instances.

## Configuration

When creating a Daemon, use the `DaemonBuilder` object to set options for the structure.
Here are the available options and fields you can use in the builder object:

- `chain` (*required*) specifies the chain the `daemon` object will interact with. <a href="https://docs.rs/cw-orch-daemon/latest/cw_orch_daemon/sync/struct.DaemonBuilder.html#method.chain" target="_blank">Documentation Link</a>
- `deployment_id` (*optional*) is used when loading and saving blockchain state (addresses and code-ids). It is useful when you have multiple instances of the same contract on a single chain. It will allow you to keep those multiple instances in the same state file without overriding state.<a href="https://docs.rs/cw-orch-daemon/latest/cw_orch_daemon/sync/struct.DaemonBuilder.html#method.deployment_id" target="_blank">Documentation Link</a>
- `handle` (*optional*) is the `tokio` runtime handled used to await async functions. `cw-orch` provides a default runtime if not specified. <a href="https://docs.rs/cw-orch-daemon/latest/cw_orch_daemon/sync/struct.DaemonBuilder.html#method.handle" target="_blank">Documentation Link</a>
- `mnemonic` (*optional*) is the mnemonic that will be used to create the sender associated with the resulting `Daemon` Object. It is not compatible with the `sender` method. <a href="https://docs.rs/cw-orch-daemon/latest/cw_orch_daemon/sync/struct.DaemonBuilder.html#method.mnemonic" target="_blank">Documentation Link</a>
- `sender` (*optional*) is the sender that will be uses with the `resulting` Daemon Object. It is not compatible with the `mnemonic` method. <a href="https://docs.rs/cw-orch-daemon/latest/cw_orch_daemon/sync/struct.DaemonBuilder.html#method.mnemonic" target="_blank">Documentation Link</a>
- `authz_granter` (*optional*) allows you to use the authz module. If this field is specified, the sender will send transactions wrapped inside an authz message sent by the specified `granter`. <a href="https://docs.cosmos.network/v0.46/modules/authz/" target="_blank">More info on the authz module</a>. <a href="https://docs.rs/cw-orch-daemon/latest/cw_orch_daemon/sync/struct.DaemonBuilder.html#method.authz_granter" target="_blank">Documentation Link</a>
- `fee_granter` (*optional*) allows you to use the fee-grant module. If this field is specified, the sender will try to pay for transactions using the specified `granter`. <a href="https://docs.cosmos.network/v0.46/modules/feegrant/" target="_blank">More info on the fee grant module</a>. <a href="https://docs.rs/cw-orch-daemon/latest/cw_orch_daemon/sync/struct.DaemonBuilder.html#method.fee_granter" target="_blank">Documentation Link</a>
- `hd_index` (*optional*) allows to set the index of the HD path for the account associated with the `Daemon` object. <a href="https://hub.cosmos.network/main/resources/hd-wallets.html" target="_blank">More info on the derivation path and index</a>. <a href="https://docs.rs/cw-orch-daemon/latest/cw_orch_daemon/sync/struct.DaemonBuilder.html#method.hd_index" target="_blank">Documentation Link</a>

> **NOTE**: if none of `sender` or `mnemonic` is specified, [env variables](../contracts/env-variable.md) will be used to construct the sender object.

Keep in mind that those options can't be changed once the `Daemon` object is built, using the `build` function. It is possible to create a new `DaemonBuilder` structure from a `Daemon` object by using the `rebuild` method and specifying the options that you need to change.

## Additional tools

The `Daemon` environment provides a bunch of tools for you to interact in a much easier way with the blockchain. Here is a non-exhaustive list:

- Send usual transactions:
  ```rust
{{#include ../../../cw-orch-daemon/examples/daemon-capabilities.rs:send_tx}}
  ```
  
- Send any transaction type registered with `cosmrs`: 
  ```rust
{{#include ../../../cw-orch-daemon/examples/daemon-capabilities.rs:cosmrs_tx}}
  ```

- Send any type of transactions (Using an `Any` type): 
  ```rust
{{#include ../../../cw-orch-daemon/examples/daemon-capabilities.rs:any_tx}}
  ```

- Simulate a transaction without sending it
  ```rust
{{#include ../../../cw-orch-daemon/examples/daemon-capabilities.rs:simulate_tx}}
    ```

## Queries

The daemon object can also be used to execute queries to the chains we are interacting with. 
This opens up a lot more applications to cw-orchestrator as this tools can also be used to manage off-chain applications.

Querying the chain for data using a daemon looks like: 
```rust,ignore
{{#include ../../../cw-orch/examples/queries/bank_query.rs:daemon_balance_query}}
```

For more information and queries, <a href="https://docs.rs/crate/cw-orch/latest/source/src/daemon/queriers.rs" target="_blank">visit the daemon querier implementations directly</a>


## Example of code leveraging Daemon capabilities

Here is an example of a script that deploys the counter contract only after a specific block_height.

```rust,ignore
{{#include ../../../contracts/counter/src/interface.rs:daemon}}
```