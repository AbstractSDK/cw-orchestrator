# Writing and Executing Scripts

Now that we have the interface written for our contract, we can start writing scripts to deploy and interact with it on a real blockchain. We'll do this by adding a `bin` folder in our contract and add our deploy script there.

## Setup

Before we get going we need to add the `bin` folder and tell cargo that it contains scripts. We can do this by creating a folder named bin in `counter` and creating a file in it called `deploy.rs`

```bash
mkdir counter/bin
touch counter/bin/deploy.rs
```

Then we want to add a new feature to our crate. We will call the feature `deploy` and it will enable interface feature as well as setting the `daemon` feature on `cw-orch`.

```toml
[features]
# ...
deploy = ["interface", "cw-orch/daemon", "dotenv", "env_logger"]


[dependencies]
# ...
# Deps for deployment
dotenv = { version = "0.15.0", optional = true } # Enables loading of .env files
env_logger = { version = "0.10.0", optional = true } # Enables logging to stdout
```

Finally, we need to add the bin to our `Cargo.toml` file. Add put a feature requirement on it:

```toml
[[bin]]
name = "deploy"
path = "bin/deploy.rs"
required-features = ["deploy"]
```

Now we're ready to start writing our script.

## Main Function

With the setup done, we can start writing our script. Our initial plan is to deploy the counter contract to the chain. We'll start by writing a main function that will call our deploy function.

```rust,ignore
{{#include ../../../contracts/counter/bin/deploy.rs}}
```

## Synchronous Daemon

## Asynchronous Daemon
