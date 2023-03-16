# Interfaces

To get started with BOOT, create a new folder in your project's package directory and add it to the workspace members.

```shell
cd packages
cargo init --lib interfaces
cd interfaces
```

Now add [boot-core](https://crates.io/crates/boot-core) to `Cargo.toml` along with the package that contains the contract's endpoint messages.

```bash
cargo add boot-core
cargo add --path ../contracts
```

```toml
[dependencies]
boot-core = "0.8.0" # latest version as of writing this article
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

In your new file, define a struct for your contract interface and provide the [`Instantiate`|`Execute`|`Query`|`Migrate`] messages to the `boot_contract` macro, which will generate fully-typed instantiate, execute, query, and migrate methods for this struct.

```rust
use boot_core::*;
use my_project::my_contract::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg};

#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct MyContract<Chain>;
```

The generic `<Chain>` argument allows you to write functions for your interface that will be executable in different environments.

> *If your entry point messages have any generic arguments, pull them out into new types before passing them into the macro.*

## Constructor

Next, you'll want to define the constructor for the interface we just defined:

```rust
impl<Chain: BootEnvironment> MyContract<Chain> {
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
            .with_wasm_path(wasm_path),
            // Mocked environments are also available and can be used for integartion testing... See Integration Testing
        )
    }
}
```

> See [Integration Testing](../integration-tests.md) for details on using mocks for integration testing.

Notice that we build the `Contract` instance and point it to the contract code using `with_wasm_path(...)`, where we provide the contract name `"my-contract"`.
This contract name will be used to search the artifacts directory (set by `ARTIFACTS_DIR` env variable) for a `my-contract.wasm` file.

Alternatively you can specify a path to the wasm artifact after running `RUSTFLAGS='-C link-arg=-s' cargo wasm` in the contract's directory. See the [CosmWasm documentation on compiling your contract](https://docs.cosmwasm.com/docs/1.0/getting-started/compile-contract/) for more information.

## Functions

Now we can start writing executable functions for our contracts with ensured type safety.
We can define functions that are generic or that can only be used called in a specific environment.
The environments that are currently supported are:

1. [cw-multi-test](https://crates.io/crates/cw-multi-test)
2. Blockchain daemons [junod](https://github.com/CosmosContracts/juno), [osmosisd](https://github.com/osmosis-labs/osmosis),...

### Generic function

Generic functions can be executed over any environment.

```rust
impl<Chain: BootEnvironment> MyContract<Chain> {
    pub fn deploy(&self, instantiate_msg: &InstantiateMsg) -> Self {
        let sender = &self.chain.sender();
        self.upload()?;
        let resp = self.instantiate(&instantiate_msg, Some(sender), None)?;
        let my_contract_address = resp.instantiated_contract_address()?;
        log::info!("deployed my-contract to {}", my_contract_address);
    }
}
```

### Daemon-only functions

```rust
impl MyContract<Daemon> {
    pub fn send_ibc_transaction(&self, msg: &ExecuteMsg) -> Self {
        let resp = self.execute(&msg,None)?;
        let destination_port = resp.event_attr_value("ibc_transfer", "destination_port");?;
    }
}
```

### Mock-only functions

```rust
impl MyContract<Mock> {
    pub fn call_other_chain (&self, msg: &ExecuteMsg) -> Self {
        let resp = self.execute(&msg,None)?;
        let destination_port = resp.event_attr_value("ibc_transfer", "destination_port");?;
    }
}
```

### Endpoint function generation

We can expand on this functionality with a simple macro that generates a contract's endpoints as callable functions. This functionality is only available if you have access to the message structs.
> You will want to feature-flag the function generation to prevent BOOT entering as a dependency when building your contract.

Here's an example:

```rust
#[cw_serde]
#[cfg_attr(feature = "boot", derive(boot_core::ExecuteFns))]
pub enum ExecuteMsg{
    Freeze {},
    UpdateAdmins { admins: Vec<String> },
    // Indicates that the call expects funds `Vec<Coin>`
    #[payable]
    Deposit {}
}

// If we now define a BOOTable contract with this execute message
#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct MyContract<Chain>;

// Then the message variants are available as functions on the struct through an "ExecuteFns" trait.
impl<Chain: BootEnvironment + Clone> MyContract<Chain> {
    pub fn test_macro(&self) -> Result<(),BootError> {
        self.freeze()?;
        self.update_admins(vec![])?;
        self.deposit(&[Coin::new(13,"juno")])?;
        Ok(())
    }
}
```

Generating query functions is a similar process but has the added advantage of using the `cosmwasm-schema` return tags to expect the correct return type.

```rust
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses)]
#[cfg_attr(feature = "boot", derive(boot_core::QueryFns))]
pub enum QueryMsg {
    /// Returns [`InfoResponse`]
    #[returns(InfoResponse)]
    Info {},
}

#[cosmwasm_schema::cw_serde]
pub struct InfoResponse {
    pub admin: Addr,
}

// If we now define a BOOTable contract with this execute message
#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct MyContract<Chain>;

// Then the message variants are available as functions on the struct through an "ExecuteFns" trait.
impl<Chain: BootEnvironment + Clone> MyContract<Chain> {
    pub fn test_macro(&self) -> Result<(),BootError> {
        // No need to specify returned type!
        let info = self.info()?;
        let admin: Addr = info.admin;
        Ok(())
    }
}

```

#### Refinement

You can also refine your contract interface manually to add more complex interactions.

```rust
// interfaces/src/my_contract.rs
// Import the boot traits
use boot_core::interface::*;
// ...

impl<Chain: BootEnvironment> MyContract<Chain> {
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

Got questions? Join the [Abstract Discord](https://discord.gg/vAQVnz3tzj) and ask in the `#boot` channel.
Learn more about Abstract at [abstract.money](https://abstract.money).

## References

- [Boot Core](https://crates.io/crates/boot-core)
- [Boot Cw-plus](https://crates.io/crates/boot-cw-plus)
- [Abstract Contract Interfaces](https://crates.io/crates/abstract-boot)
