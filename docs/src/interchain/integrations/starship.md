# Local multi-chain setup

For testing, you might want to interact with local nodes and relayers to test IBC interactions. To do so, we allow users to leverage <a href="https://docs.cosmology.zone/starship" target="_blank">@cosmology-tech/Starship</a>. Starship allows developers to spin up a fully simulated mini-cosmos ecosystem. It sets up Cosmos SDK Nodes as well as relayers between them allowing you to focus on your application and less on the testing environment. [Read more on how to setup Starship](./starship.md) and make it work with `cw-orchestrator`.

## Setup Starship

You can find helpers to setup starship in the [`cw-orch` repo](https://github.com/AbstractSDK/cw-orchestrator/tree/main/packages/interchain/starship/starship). Here are the commands to launch in order to have starship up and running:

- Install Starship and the chain+relayer cluster (do this once, this make take a little time)

    ```bash
    make setup
    ```

- Start Starship. This will create all chains and relayers based on the [example configuration file](https://github.com/AbstractSDK/cw-orchestrator/blob/main/packages/interchain/starship/examples/starship.yaml).

    ```bash
    make install
    ```

Starship will most likely crash after at most 1 day of usage. Don't forget to execute `make stop`, wait for everything to be stopped and `make install` from time to time to restart the whole chain cluster.

In case you run into issues when using and interacting with Starship, you might need to run the `port-forward.sh` script, after a successful `make install`.

## Create the Connector in Rust Code

When the starship instance is up and running, the starship adapter that `cw-orchestrator` provides will detect the deployment and create the right variables and structures for you to interact and test with.

```rust,ignore
use cw_orch_interchain::interchain::{Starship, ChannelCreator};

# fn main(){
let starship = Starship::new(None)?;
let interchain = starship.interchain_env();

let local_juno = interchain.chain("juno-1")?;
let local_osmo = interchain.chain("osmosis-1")?;
# }
```

> **NOTE**: The argument of the `Starship::new` function is the optional URL of the starship deployment. It defaults to `http://localhost:8081`, but you can customize it if it doesn't match your setup. All the starship data, daemons and relayer setup is loaded from that URL.

## Usage

Once the two steps above have been completed, you can [use starship](../quick-start.md) as every other interchain environment. Refer to the [interchain Daemon page](./daemon.md) to understand how it works under the hood.
