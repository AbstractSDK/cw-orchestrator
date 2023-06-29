<!-- add bg image -->
<div align="center">
  <img src="https://raw.githubusercontent.com/AbstractSDK/assets/mainline/orchestrator_bg2.png", width = "230px"/>

# cw-orchestrator

<a href="https://docs.rs/cw-orch/latest" ><img alt="docs.rs" src="https://img.shields.io/docsrs/cw-orch"></a> <a href="https://crates.io/crates/cw-orch" ><img alt="Crates.io" src="https://img.shields.io/crates/d/cw-orch"></a> <a href="https://app.codecov.io/gh/AbstractSDK/cw-orchestrator" ><img alt="Codecov" src="https://img.shields.io/codecov/c/github/AbstractSDK/cw-orchestrator?token=CZZH6DJMRY"></a>

</div>

A Rust tool for interacting with [CosmWasm](https://cosmwasm.com/) smart contracts. It provides a type-safe interface to CosmWasm contracts and allows you to easily interact with them. It does this by providing a set of macros that generate type-safe interfaces to your contracts. You can then combine your contract interfaces into a single object that can be shared with others to ease integration efforts and encourage collaboration.

The documentation here gives you a brief overview of the functionality that cw-orchestrator provides. We provide more documentation at [orchestrator.abstract.money](https://orchestrator.abstract.money).

## How it works

Interacting with a [CosmWasm](https://cosmwasm.com/) contract is done by calling the contract's endpoints using the appropriate message for that endpoint (`ExecuteMsg`,`InstantiateMsg`, `QueryMsg`, `MigrateMsg`, etc.). cw-orchestrator generates typed interfaces for your contracts, allowing them to be type-checked at compile time. This generic interface then allows you to write environment-generic code, meaning that you can re-use the code that you write to deploy your application to `cw-multi-test` when deploying to test/mainnet.

## Maintained Interfaces

We maintain a small set of interfaces ourselves that we use in our own projects. These interfaces are maintained by the Abstract team and are a good reference for how to use the library.

| Codebase | Latest Version |
|---|---|
| [cw-plus](https://github.com/AbstractSDK/cw-plus) | <img alt="GitHub tag (latest SemVer)" src="https://img.shields.io/github/v/tag/AbstractSDK/cw-plus"> |
| [wyndex](https://github.com/AbstractSDK/integration-bundles) | <img alt="GitHub tag (latest SemVer)" src="https://img.shields.io/github/v/tag/AbstractSDK/integration-bundles"> |
| [AbstractSDK](https://github.com/AbstractSDK/contracts/tree/main/packages/abstract-interface) | <img alt="Crates.io" src="https://img.shields.io/crates/v/abstract-interface"> |

## Creating an Interface

In order to generate a typed interface to your contract you can either pass the contract's message types into the `contract` macro or you can add the `interface` macro to your endpoint function exports!

### The `contract` Macro

Provide your messages to a new struct that's named after your contract.

```rust
use cw_orch::interface;
use cw20_base::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg};

// Provide the messages in the order Init, Exec, Query, Migrate.
#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20;
```

The macro will generate a `new` function that takes the contract name and the chain that you want to interact with. You can then use this interface to interact with the contract.

### The `interface_entry_point` Macro

You create a contract interface by adding the `interface_entry_point` macro to your contract endpoints. The name of the generated interface will be the crate name in PascalCase.

```ts
use cw_orch::interface_entry_point;

#[cw_orch::interface_entry_point]
fn instantiate(...)

#[cw_orch::interface_entry_point]
fn execute(...)
```

You now have a contract interface that you can use to interact with your contract.

### Usage

You can use this interface to interact with the contract:

```rust
use cw_orch::interface;
use cw_orch::prelude::*;
use cw20::{Cw20Coin, BalanceResponse};
use cw20_base::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20;

// Implement the Uploadable trait so it can be uploaded to the mock. 
impl <Chain: CwEnv> Uploadable for Cw20<Chain> {
    fn wrapper(&self) -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            )
            .with_migrate(cw20_base::contract::migrate),
        )
    }
}

// ## Environment setup ##

let sender = Addr::unchecked("sender");
// Create a new mock chain (backed by cw-multi-test)
let chain = Mock::new(&sender);

// Create a new Cw20 interface
let cw20_base: Cw20<Mock> = Cw20::new("my_token", chain);

// Upload the contract
cw20_base.upload().unwrap();

// Instantiate a CW20 token
let cw20_init_msg = InstantiateMsg {
    decimals: 6,
    name: "Test Token".to_string(),
    initial_balances: vec![Cw20Coin {
        address: sender.to_string(),
        amount: 10u128.into(),
    }],
    marketing: None,
    mint: None,
    symbol: "TEST".to_string(),
};
cw20_base.instantiate(&cw20_init_msg, None, None).unwrap();

// Query the balance
let balance: BalanceResponse = cw20_base.query(&QueryMsg::Balance { address: sender.to_string() }).unwrap();

assert_eq!(balance.balance.u128(), 10u128);
```

## Features

cw-orchestrator provides two additional macros that can be used to improve the scripting experience.

### ExecuteFns

The `ExecuteFns` macro can be added to the `ExecuteMsg` definition of your contract. It will generate a trait that allows you to call the variants of the message directly without the need to construct the struct yourself.

The macros should only be added to the structs when the "interface" trait is enable. This is ensured by the `interface` feature in the following example

Example:

```rust
use cw_orch::prelude::*;

#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    Freeze {},
    UpdateAdmins { admins: Vec<String> },
    /// the `payable` attribute will add a `coins` argument to the generated function
    #[payable]
    Deposit {}
}

// Define the interface, which is generic over the CosmWasm environment (Chain)
#[cw_orch::interface(Empty,ExecuteMsg,Empty,Empty)]
struct Cw1<Chain>;

impl<Chain: CwEnv> Cw1<Chain> {
    pub fn test_macro(&self) {
        self.freeze().unwrap();
        self.update_admins(vec!["new_admin".to_string()]).unwrap();
        self.deposit(&[Coin::new(13,"juno")]).unwrap();
    }
}
```

> We recommend shielding the `ExecuteMsgFns` macro behind a feature flag to avoid pulling in `cw-orchestrator` by default.

### QueryFns

The `QueryFns` derive macro works in the same way as the `ExecuteFns` macro but it also uses the `#[returns(QueryResponse)]` attribute from `cosmwasm-schema` to generate the queries with the correct response types.

### `impl_into` Attribute

For nested messages (execute and query) you can add an `impl_into` attribute. This expects the enum to implement the `Into` trait for the provided type. This is extremely useful when working with generic messages:

```rust
use cw_orch::interface;
use cw_orch::prelude::*;

// An execute message that is generic.
#[cosmwasm_schema::cw_serde]
pub enum GenericExecuteMsg<T> {
    Generic(T),
}

// Now the following is possible:
type ExecuteMsg = GenericExecuteMsg<Foo>;

#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum Foo {
    Bar { a: String },
}

impl From<Foo> for ExecuteMsg {
    fn from(msg: Foo) -> Self {
        ExecuteMsg::Generic(msg)
    }
}

#[interface(Empty, ExecuteMsg, Empty, Empty)]
struct Example<Chain>;

impl<Chain: CwEnv> Example<Chain> {
    pub fn test_macro(&self) {
        // function `bar` is available because of the `impl_into` attribute!
        self.bar("hello".to_string()).unwrap();
    }
}
```

## Contributing

We'd really appreciate your help! Please read our [contributing guidelines](docs/src/contributing.md) to get started.

## Documentation

The documentation is generated using [mdbook](https://rust-lang.github.io/mdBook/index.html). Edit the files in the `docs/src` folder and run

```shell
just serve-docs
```

to view the changes.

[Release Docs](https://orchestrator.abstract.money)
[Dev Docs](https://dev-orchestrator.abstract.money)

## Testing

To test the full application you can run the following command:

```shell
cargo test --jobs 1 --all-features
```

## References

Enjoy scripting your smart contracts with ease? Build your contracts with ease by using [Abstract](https://abstract.money).

## Disclaimer

This software is provided as-is without any guarantees.

## Credits

cw-orchestrator is inspired by [terra-rust-api](https://github.com/PFC-Validator/terra-rust) and uses [cosmos-rust](https://github.com/cosmos/cosmos-rust) for [protocol buffer](https://developers.google.com/protocol-buffers/docs/overview) gRPC communication.
