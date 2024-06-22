<!-- add bg image -->
<div align="center">
  <img src="https://raw.githubusercontent.com/AbstractSDK/assets/mainline/v1/orchestrator_bg2.png", width = "230px"/>

# cw-orchestrator

<a href="https://docs.rs/cw-orch/latest" ><img alt="docs.rs" src="https://img.shields.io/docsrs/cw-orch"></a> <a href="https://crates.io/crates/cw-orch" ><img alt="Crates.io" src="https://img.shields.io/crates/d/cw-orch"></a> <a href="https://app.codecov.io/gh/AbstractSDK/cw-orchestrator" ><img alt="Codecov" src="https://img.shields.io/codecov/c/github/AbstractSDK/cw-orchestrator?token=CZZH6DJMRY"></a>

</div>

A Rust tool for interacting with [CosmWasm](https://cosmwasm.com/) smart contracts. It provides a type-safe interface to CosmWasm contracts and allows you to easily interact with them. It does this by providing a set of macros that generate type-safe interfaces to your contracts. You can then combine your contract interfaces into a single object that can be shared with others to ease integration efforts and encourage collaboration.

The documentation here gives you a brief overview of the functionality that cw-orchestrator provides. We provide more documentation at [orchestrator.abstract.money](https://orchestrator.abstract.money).

## How it works

Interacting with a [CosmWasm](https://cosmwasm.com/) contract involves calling the contract's endpoints using the appropriate message for that endpoint (`ExecuteMsg`,`InstantiateMsg`, `QueryMsg`, `MigrateMsg`, etc.). cw-orchestrator generates typed interfaces for your contracts, allowing them to be type-checked at compile time. This generic interface then allows you to write environment-generic code, meaning that you can re-use the code that you write to deploy your application to `cw-multi-test` when deploying to test/mainnet.

## Maintained Interfaces

We maintain a small set of interfaces ourselves that we use in our own projects. These interfaces are maintained by the Abstract team and are a good reference for how to use the library.

| Codebase | Latest Version |
|---|---|
| [cw-plus](https://github.com/AbstractSDK/cw-plus) | <img alt="GitHub tag (latest SemVer)" src="https://img.shields.io/github/v/tag/AbstractSDK/cw-plus"> |
| [AbstractSDK](https://github.com/AbstractSDK/contracts/tree/main/packages/abstract-interface) | <img alt="Crates.io" src="https://img.shields.io/crates/v/abstract-interface"> |

## Creating an Interface

In order to generate a typed interface to your contract you can pass the contract's message types into the `cw_orch::interface` macro.

### The `interface` Macro

Provide your messages to a new struct that's named after your contract.

```rust,ignore
use cw_orch::interface;
use cw20_base::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg};

// Provide the messages in the order Init, Exec, Query, Migrate.
#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20;
```

The macro will generate a `new` function that takes the contract name and the chain that you want to interact with. You can then use this interface to interact with the contract.

### Usage

You can use this interface to deploy and interact with the contract:

```rust,ignore
use cw_orch::interface;
use cw_orch::prelude::*;
use cw20::{Cw20Coin, BalanceResponse};

// Implement the Uploadable trait so it can be uploaded to the mock. 
impl <Chain> Uploadable for Cw20<Chain> {
    fn wrapper() -> Box<dyn MockContract<Empty>> {
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


fn example_test() {
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
}
```

## Features

`cw-orchestrator` provides two additional macros that can be used to improve the scripting experience.

### ExecuteFns

The `ExecuteFns` macro can be added to the `ExecuteMsg` definition of your contract. It will generate a trait that allows you to call the variants of the message directly without the need to construct the struct yourself.

The `ExecuteFns` macro would only run on the Msg when compiled for non-wasm target. Optionally you can ensure it by the `interface` feature, like in the following example:

```rust,ignore
use cw_orch::prelude::*;

#[cosmwasm_schema::cw_serde]
// ⬇️ This feature flag prevents cw-orchestrator from entering your contract.
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
pub enum ExecuteMsg {
    Freeze {},
    UpdateAdmins { admins: Vec<String> },
    /// the `payable` attribute can be used to add a `coins` argument to the generated function.
    #[cw_orch(payable)]
    Deposit {}
}
```

The generated functions can then be used for any interface that uses this `ExecuteMsg`.

```rust,ignore
// Define the interface, which is generic over the CosmWasm environment (Chain)
#[cw_orch::interface(Empty,ExecuteMsg,Empty,Empty)]
struct Cw1<Chain>;

impl<Chain> Cw1<Chain> {
    pub fn test_macro(&self) {
        // Enjoy the nice API! 
        self.freeze().unwrap();
        self.update_admins(vec!["new_admin".to_string()]).unwrap();
        self.deposit(&[Coin::new(13,"juno")]).unwrap();
    }
}
```

### QueryFns

The `QueryFns` derive macro works in the same way as the `ExecuteFns` macro but it also uses the `#[returns(QueryResponse)]` attribute from `cosmwasm-schema` to generate the queries with the correct response types.

### Nested types

For nested messages (execute and query), you just need to derive `ExecuteFns` or `QueryFns` on the underlying structures. In general, every structure that implements the `Into` trait for the contract message will make the function available on the contract. To make that clearer, her's an example:

```rust,ignore
use cw_orch::interface;
use cw_orch::prelude::*;

// An execute message that is generic.
#[cosmwasm_schema::cw_serde]
pub enum GenericExecuteMsg<T> {
    Generic(T),
}

// A type that will fill the generic.
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum Foo {
    Bar { a: String },
}


// Now we construct the concrete type with `Foo` in place of the generic.
type ExecuteMsg = GenericExecuteMsg<Foo>;
// And we implement the `From` trait (which auto-implements `Into`).
impl From<Foo> for ExecuteMsg {
    fn from(msg: Foo) -> Self {
        ExecuteMsg::Generic(msg)
    }
}

#[interface(Empty, ExecuteMsg, Empty, Empty)]
struct Example<Chain>;

impl<Chain: CwEnv> Example<Chain> {
    pub fn test_macro(&self) {
        // Function `bar` is available because `Foo` implements `Into<GenericExecuteMsg<Foo>>`
        self.bar("hello".to_string()).unwrap();
    }
}
```

### Testing with OsmosisTestTube

[OsmosisTestTube](https://github.com/osmosis-labs/test-tube) is available for testing in cw-orchestrator. In order to use it, you may need to install [clang](https://clang.llvm.org/) and [go](https://go.dev/) to compile the osmosis blockchain that serves as the backend for this env. This compilation is taken care of by cargo directly but if you don't have the right dependencies installed, weird errors may arise.

- Visit <https://docs.osmosis.zone/osmosis-core/osmosisd> for a comprehensive list of dependencies.
- Visit [the INSTALL.md file](./INSTALL.md) for a list of dependencies we have written specifically for use with cw-orch.  

## Supported chains

Cw-orchestrator supports the following chains natively:
🟥 LocalNet, 🟦 Testnet, 🟩 Mainnet

- Archway 🟦🟩
- Injective 🟦🟩
- Juno 🟥🟦🟩
- Kujira 🟦
- Migaloo 🟥🟦🟩
- Neutron 🟦🟩
- Nibiru 🟦
- Osmosis 🟥🟦🟩
- Sei 🟥🟦🟩
- Terra 🟥🟦🟩
- Rollkit 🟥🟦
- Xion 🟦

Additional chains can easily be integrated by creating a new [`ChainInfo`](./packages/cw-orch-networks/src/chain_info.rs) structure. This can be done in your script directly. If you have additional time, don't hesitate to open a PR on this repository.

## Installation

Cw-orch relies on external tools to work properly. Visit [the INSTALL.md file](./INSTALL.md) for a list of dependencies and install commands that are needed for cw-orch to work properly on your machine.

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
