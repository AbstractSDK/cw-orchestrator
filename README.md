<!-- ![alt text](https://raw.githubusercontent.com/Abstract-OS/assets/c85b8ed5104b26bfb0f97dc9d30a8813a4a1b60b/DALL%C2%B7E%20Boot%20(2).png) -->
# BOOT (Unstable API)

Multi-environment [CosmWasm](https://cosmwasm.com/) smart-contract scripting library.  Documentation is available at [https://boot.abstract.money](https://boot.abstract.money).

> [BOOT](boot-core/README.md) is inspired by [terra-rust-api](https://github.com/PFC-Validator/terra-rust) and uses [cosmos-rust](https://github.com/cosmos/cosmos-rust) for [protocol buffer](https://developers.google.com/protocol-buffers/docs/overview) gRPC communication.


[boot-cw-plus](boot-cw-plus/README.md) uses BOOT to provide standard type-safe interfaces for interacting with [cw-plus](https://github.com/CosmWasm/cw-plus) contracts.

The use of this software makes it easier to quickly deploy and iterate on your contracts. You should use this function responsibly when working on mainnet or testnet as the code you upload to those networks takes up valuable space. It is strongly suggested to use a locally-hosted daemon like [localterra](https://github.com/terra-money/LocalTerra), [local junod](https://docs.junonetwork.io/smart-contracts-and-junod-development/junod-local-dev-setup), etc.
.
## How it works

Interacting with a [CosmWasm](https://cosmwasm.com/) is possible through the contract's endpoints using the appropriate message for that endpoint (`ExecuteMsg`,`InstantiateMsg`, `QueryMsg`, `MigrateMsg`, etc.).

In order to perform actions on the contract you can define a struct for your contract, passing the contract's entry point types into the `boot_contract` macro:

```rust
#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct MyContract<Chain>;
```

The macro implements a set of traits for the struct. These traits contain functions that we can use to interact with the contract and they prevent us from executing a faulty message on a contract. The implementation for a CW20 token is shown below. The full implementation resides [here](boot-cw-plus/src/cw20.rs)

```rust
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw20;
```

You can now perform any action on the cw20 contract and implement custom actions.

```rust
    let cw20_token = Cw20::new(chain)?;
    let msg = ExecuteMsg::Transfer {
            recipient,
            amount: amount.into(),
        };
    cw20_token.execute(&msg, None)?;
    let token_info: TokenInfoResponse = cw20_token.query(&Cw20QueryMsg::TokenInfo {}).await?;
```

We would recommend reading through [the full cw20 executable example here](boot-core/examples/cw20.rs).

## Advanced features

BOOT provides two additional macros that can be used to improve the scripting experience. 

### ExecuteFns
The `ExecuteFns` macro can be added to the `ExecuteMsg` definition of your contract. It will generate a trait that allows you to call the variants of the message directly without the need to construct the struct itself. 

Example:
```rust
#[cw_serde]
#[derive(ExecuteFns)]
pub enum ExecuteMsg{
    /// Freeze will make a mutable contract immutable, must be called by an admin
    Freeze {},
    /// UpdateAdmins will change the admin set of the contract, must be called by an existing admin,
    /// and only works if the contract is mutable
    UpdateAdmins { admins: Vec<String> },
    #[payable]
    Deposit {}
}

#[boot_contract(Empty,ExecuteMsg,Empty,Empty)]
struct Cw1

impl<Chain: BootEnvironment> Cw1<Chain> {
    pub fn test_macro(&self) {
        self.freeze().unwrap();
        self.update_admins(vec![]).unwrap(); 
        self.deposit(&[Coin::new(13,"juno")]).unwrap();
    }
}
```
For nested execute messages see [this PR](https://github.com/Abstract-OS/BOOT/pull/40). 

### QueryFns 

The `QueryFns` derive macro works in the same way as the `ExecuteFns` macro but it also uses the `#[returns(QueryResponse)]` attribute from `cosmwasm-schema` to generate the queries with the correct response types.

# Contributing
Feel free to open issues or PRs!

## Documentation
The documentation is generated using [mdbook](https://rust-lang.github.io/mdBook/index.html). Edit the files in the `docs/src` folder and run
```shell
cd docs && mdbook serve --open --port 5000
```
to view the changes.

# References
Enjoy scripting your smart contracts with ease? Build your contracts with ease with [Abstract](https://abstract.money). Get started [here](https://docs.abstract.money/building-on-abstract/writing-modules/writing-an-app).

# Disclaimer
This software is provided as-is without any guarantees.
