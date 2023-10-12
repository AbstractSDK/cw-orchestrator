# Writing and Executing Scripts

Now that we have the interface written for our contract, we can start writing scripts to deploy and interact with it on a real blockchain. We'll do this by adding a `examples` folder in [the counter contract](https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter) and add our deploy script there.

## Setup

Before we get going we need to add the `examples` folder and tell cargo that it contains scripts. We can do this by creating a folder named examples in `counter` and creating a file in it called `deploy.rs`

```bash
mkdir counter/examples
touch counter/examples/deploy.rs
```

Then we want to add the required dependencies to the `dev-dependencies` section of our `Cargo.toml` file. We'll need `dotenv` to load our environment variables and `pretty_env_logger` to log to stdout. We're using `examples` instead of `bin` because setting a feature on an optional dependency is not supported.

```toml
[dev-dependencies]
# Deps for deployment
dotenv = { version = "0.15.0" } # Enables loading of .env files
pretty_env_logger = { version = "0.5.0" } # Enables logging to stdout and prettifies it
counter-contract = { path = ".", features = ["interface"] } # Allows to activate the `interface` feature for examples and tests 
```

Now we're ready to start writing our script.

## Main Function

With the setup done, we can start writing our script. Our initial plan is to deploy the counter contract to the chain. We'll start by writing a main function that will call our deploy function.

We start by creating the chain object for specifying we want to interact with a local juno instance : 

```rust,ignore
{{#include ../../../contracts/counter/examples/deploy.rs:chain_construction}}
```

Then we can interact with our contract using that chain

```rust,ignore
{{#include ../../../contracts/counter/examples/deploy.rs:contract_interaction}}
```

## Asynchronous Daemon

All the functionalities described in this guide/tutorial only allow for synchronous interactions. If for whatever reason you need to interact with an actual chain in an asynchronous way, you can use the [`DaemonAsync`](https://docs.rs/cw-orch/latest/cw_orch/daemon/struct.DaemonAsync.html) structure. However, this structure won't allow for much interoperability as it's not compatible with the `cw-orch` Contract structure. Blockchain transactions have to be sequential because of the `sequence` of an account and that's why we provide limited support to asynchronous transaction broadcasting capabilities. 