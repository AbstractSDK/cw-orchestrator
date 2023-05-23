# Writing and Executing Scripts

Now that we have the interface written for our contract, we can start writing scripts to deploy and interact with it on a real blockchain. We'll do this by adding a `examples` folder in our contract and add our deploy script there.

## Setup

Before we get going we need to add the `examples` folder and tell cargo that it contains scripts. We can do this by creating a folder named examples in `counter` and creating a file in it called `deploy.rs`

```bash
mkdir counter/examples
touch counter/examples/deploy.rs
```

Then we want to add the required dependencies to the `dev-dependencies` section of our `Cargo.toml` file. We'll need `dotenv` to load our environment variables and `env_logger` to log to stdout. We're using `examples` instead of `bin` because setting a feature on an optional dependency is not supported.

```toml
[dev-dependencies]
# Deps for deployment
dotenv = { version = "0.15.0" } # Enables loading of .env files
env_logger = { version = "0.10.0" } # Enables logging to stdout
```

Finally, we need to add the examples to our `Cargo.toml` file. Add put a feature requirement on it:

```toml
[[example]]
name = "deploy"
path = "examples/deploy.rs"
```

Now we're ready to start writing our script.

## Main Function

With the setup done, we can start writing our script. Our initial plan is to deploy the counter contract to the chain. We'll start by writing a main function that will call our deploy function.

```rust,ignore
{{#include ../../../contracts/counter/examples/deploy.rs}}
```

## Synchronous Daemon

## Asynchronous Daemon
