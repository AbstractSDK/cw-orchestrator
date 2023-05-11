# Interfaces

To get started with cw-orchestrator, create a new folder in your project's package directory and add it to the workspace members.

```shell
cd packages
cargo init --lib interfaces
cd interfaces
```

Now add [cw-orchestrator](https://crates.io/crates/cw-orch) to `Cargo.toml` along with the package that contains the contract's endpoint messages.

```bash
cargo add cw-orch
cargo add log # optional for logging
cargo add anyhow # optional for simple error handling
cargo add --path ../my-project
```

```toml
[dependencies]
cw-orch = "0.10.0" # latest version as of writing this article
my-project = { path = "../my-project"}
# ...other dependencies
```

## Defining Contract Interfaces

The contract interface is a struct that provides accessible methods to deploy and interact with an instance of your contract. Let's see how to use it.

First, create a new file in the src directory of the interfaces package, and add it to the library declaration

```bash
touch src/my-contract.rs
echo 'pub mod my_contract;' >> src/lib.rs
```

In your new file, define a struct for your contract interface and provide the [`Instantiate`|`Execute`|`Query`|`Migrate`] messages to the `contract` macro, which will generate fully-typed instantiate, execute, query, and migrate methods for this struct.

```rust
use cw_orch::*;
// We use pub here to be able to import those messages directly
// from the interfaces crate in the next steps (scripting, intergation tests...)
pub use my_project::my_contract::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct MyContract<Chain>;
```

> *If your entry point messages have any generic arguments, pull them out into new types before passing them into the macro.*

## Constructor

Next, you'll want to define the constructor for the interface we just defined. In order to do so, first include the contract interface (`instantiate`, `execute` and `query` functions) in your package :

```bash
cargo add --path ../../my-contract
```


The generic `<Chain>` argument allows you to write functions for your interface that will be executable in different environments.

```rust

impl<Chain: CwEnv> MyContract<Chain> {
    /// Construct a new instance of MyContract
    /// * `contract_id` - what your contract should be called in local state (*not* on-chain)
    /// * `chain` - the environment to deploy to
    pub fn new(contract_id: &str, chain: Chain) -> Self {
        // Use an absolute path
        let wasm_path = "../../target/wasm32-unknown-unknown/release/my-contract.wasm";
       // OR give the contract name and set the ARTIFACTS_DIR environment variable to the artifacts folder
       let wasm_path = "my-contract";
        Self(
            Contract::new(contract_id, chain)
            // Adds the wasm path for uploading to a node
            .with_wasm_path(wasm_path)
            // Adds the contract's endpoint functions for mocking
            .with_mock(Box::new(
                   ContractWrapper::new_with_empty(
                     my_contract::contract::execute,
                     my_contract::contract::instantiate,
                     my_contract::contract::query,
                ))),
        )
    }
}
```

Notice that we build the `Contract` instance and point it to the contract code using `with_wasm_path(...)`, where we provide the contract name `"my-contract"`.
This contract name will be used to search the artifacts directory (set by `ARTIFACTS_DIR` env variable) for a `my-contract.wasm` file.

Alternatively you can specify a path to the wasm artifact that's generated after running `RUSTFLAGS='-C link-arg=-s' cargo wasm` in the contract's directory. See the [CosmWasm documentation on compiling your contract](https://docs.cosmwasm.com/docs/1.0/getting-started/compile-contract/) for more information.

## Functions

Now you can start writing executable functions for your contracts with ensured type safety.
You can write functions that are generic or that can only be used called in a specific environment.
The environments that are currently supported are:

1. [cw-multi-test](https://crates.io/crates/cw-multi-test)
2. Blockchain daemons with CosmWasm enabled: [junod](https://github.com/CosmosContracts/juno), [osmosisd](https://github.com/osmosis-labs/osmosis),...

### Generic function

Generic functions can be executed over any environment.

```rust
impl<Chain: CwEnv> MyContract<Chain> {
    pub fn deploy(&self, instantiate_msg: &InstantiateMsg) -> Result<()> {
        let sender = &self.get_chain().sender();
        self.upload()?;
        let resp = self.instantiate(instantiate_msg, Some(&sender), None)?;
        let my_contract_address = resp.instantiated_contract_address()?;
        log::info!("deployed my-contract to {}", my_contract_address);
        Ok(())
    }
}
```


### Daemon-only functions

```rust
impl MyContract<Daemon> {
    pub fn send_ibc_transaction(&self, msg: &ExecuteMsg) -> Result<String> {
        let resp = self.execute(msg,None)?;
        let destination_port = resp.event_attr_value("ibc_transfer", "destination_port")?;
        Ok(destination_port)
    }
}
```

### Mock-only functions

```rust
impl MyContract<Mock> {
    pub fn call_other_chain (&self, msg: &ExecuteMsg) -> Result<String> {
        let resp = self.execute(msg,None)?;
        let destination_port = resp.event_attr_value("ibc_transfer", "destination_port")?;
        Ok(destination_port)
    }
}
```

### Endpoint function generation

#### Execution

We can expand on this functionality with a simple macro that provides access to a contract's endpoints as callable functions. This functionality is only available if you have access to the message structs's crate.
> You will want to feature-flag the function generation to prevent cw-orchestrator entering as a dependency when building your contract.

Here's an example with the macro shielded behind a "interface" feature flag:

```rust
#[cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
pub enum ExecuteMsg{
    Freeze {},
    UpdateAdmins { admins: Vec<String> },
    // Indicates that the call expects funds `Vec<Coin>`
    #[cfg_attr(feature = "interface", payable)]
    Deposit {}
}

// If we now define a orchestrateable contract with this execute message
#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct MyContract<Chain>;

// Then the message variants are available as functions on the struct through an "ExecuteFns" trait.
impl<Chain: CwEnv + Clone> MyContract<Chain> {
    pub fn test_macro(&self) -> Result<(),CwOrchError> {
        self.freeze()?;
        self.update_admins(vec![])?;
        self.deposit(&[Coin::new(13,"juno")])?;
        Ok(())
    }
}
```

In order for the above code to work, you will need to follow those simple steps :
1. Add the following line to your `packages/my-project.Cargo.toml`. This will allow to activate the interface feature for creating `ExecuteFns` outside of the crate
    ```cargo
    [features]
    interface=[]
    ```

2. Add the following import in your `packages/interfaces/src/my-contract.rs` file :
    ```rust
    use my_project::my_contract::ExecuteMsgFns;
    ```



#### Query

Generating query functions is a similar process but has the added advantage of using the `cosmwasm-schema` return tags to detect the query's return type.

```rust
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses)]
#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
pub enum QueryMsg {
    /// Returns [`InfoResponse`]
    #[returns(InfoResponse)]
    Info {},
}

#[cosmwasm_schema::cw_serde]
pub struct InfoResponse {
    pub admin: Addr,
}

// If we now define a orchestrateable contract with this execute message
#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct MyContract<Chain>;

// Then the message variants are available as functions on the struct through an "ExecuteFns" trait.
impl<Chain: CwEnv + Clone> MyContract<Chain> {
    pub fn test_macro(&self) -> Result<(),CwOrchError> {
        // No need to specify returned type!
        // info of type `InfoResponse` is returned
        let info = self.info()?;
        let admin: Addr = info.admin;
        Ok(())
    }
}

```

In order to derive query functions, you NEED to serive the QueryResponses crate for your QueryMsgs struct. This is mandatory in order to ensure type-safety for all messages and responses.

This time, add the following import in your `packages/interfaces/src/my-contract.rs` file :
    ```rust
    use my_project::my_contract::QueryMsgFns;
    ```


#### Refinement

You can also refine your contract interface manually to add more complex interactions.

```rust
// interfaces/src/my_contract.rs
// Import the cw-orchestrator traits
use cw_orch::interface::*;
// ...

impl<Chain: CwEnv> MyContract<Chain> {
    pub fn new(contract_id: &str, chain: Chain) -> Self {
      // ...
    }

    /// Query the balance of an address
    /// `address` - the address to query
    pub fn balance(&self, address: Addr) -> Result<BalanceResponse> {
        let balance_query = QueryMsg::Balance { address };
        self.query(balance_query)
    }

    /// Update the balance of an address
    /// `address` - the address to update
    /// `balance` - the new balance
    pub fn update_balance(&self, address: Addr, balance: Uint128) -> Result<Chain::TxResult> {
        let update_balance_msg = ExecuteMsg::UpdateBalance {
            address,
            balance,
        };
        self.execute(update_balance_msg)
    }
}

```

## Learn more

Got questions? Join the [Abstract Discord](https://discord.gg/vAQVnz3tzj) and ask in the `#cw-orchestrator` channel.
Learn more about Abstract at [abstract.money](https://abstract.money).

## References

- [cw-orchestrator](https://crates.io/crates/cw-orch)
- [cw-plus-orc](https://crates.io/crates/cw-plus-orc)
- [Abstract Contract Interfaces](https://crates.io/crates/abstract-cw-orch)
