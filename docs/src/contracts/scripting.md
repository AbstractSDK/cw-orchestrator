# Writing and Executing Scripts

Now that we have the interface written for our contract, we can start writing scripts to deploy and interact with it on a real blockchain. We'll do this by adding a `examples` folder in <a href="https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter" target="_blank">the counter contract</a> and add our deploy script there. We only provide an overview of how scripting can be done here. Find our more about how to make your scripting dreams come true on the [Daemon page](../integrations/daemon.md).

## Setup

Before we get going we need to add the `examples` folder and tell cargo that it contains scripts. We can do this by creating a folder named examples in `counter` and creating a file in it called `deploy.rs`

```bash
mkdir counter/examples
touch counter/examples/deploy.rs
```

Then we want to add the required dependencies to the `dev-dependencies` section of our `Cargo.toml` file. We'll need `dotenv` to load our environment variables and `pretty_env_logger` to log to stdout.

```toml
[dev-dependencies]
# Deps for deployment
dotenv = { version = "0.15.0" } # Enables loading of .env files
pretty_env_logger = { version = "0.5.0" } # Enables logging to stdout and prettifies it
```

Now we're ready to start writing our script.

## Main Function

With the setup done, we can start writing our script. Our initial plan is to deploy the counter contract to the chain. We'll start by writing a main function that will call our deploy function.

We start by creating the chain object for specifying we want to interact with a local juno instance:

```rust,ignore
{{#include ../../../contracts-ws/contracts/counter/examples/deploy.rs:chain_construction}}
```

Then we can interact with our contract using that chain

```rust,ignore
{{#include ../../../contracts-ws/contracts/counter/examples/deploy.rs:contract_interaction}}
```

## Asynchronous Daemon

All the functionalities described in this guide/tutorial only allow for synchronous interactions. If for whatever reason you need to interact with an actual chain in an asynchronous way, you can use the <a href="https://docs.rs/cw-orch/latest/cw_orch/daemon/struct.DaemonAsync.html" target="_blank">`DaemonAsync`</a> structure. However, this structure won't allow for much interoperability as it's not compatible with the `cw-orch` Contract structure. Blockchain transactions have to be sequential because of the `sequence` of an account and that's why we provide limited support to asynchronous transaction broadcasting capabilities.
