![alt text](https://raw.githubusercontent.com/Abstract-OS/assets/c85b8ed5104b26bfb0f97dc9d30a8813a4a1b60b/DALL%C2%B7E%20Boot%20(2).png)
# BOOT

Smart contract scripting library to ease [CosmWasm](https://cosmwasm.com/) smart contract deployment and testing.

> [BOOT](boot-core/README.md) is inspired by [terra-rust-api](https://github.com/PFC-Validator/terra-rust) and uses [cosmos-rust](https://github.com/cosmos/cosmos-rust) for [protocol buffer](https://developers.google.com/protocol-buffers/docs/overview) parsing.

[boot-plus](boot-plus/README.md) uses BOOT to provide standard type-safe interfaces to interact with [cw-plus](https://github.com/CosmWasm/cw-plus) contracts.

The use of this software makes it easier to quickly deploy and iterate on your contracts. You should use this function responsibly when working on mainnet or testnet as ALL the code you upload to those networks takes up valuable space. Therefore I strongly suggest using a locally-hosted chain like [localterra](https://github.com/terra-money/LocalTerra), [local junod](https://docs.junonetwork.io/smart-contracts-and-junod-development/junod-local-dev-setup), etc. 
.
## How it works

Usually your contracts workspace will have a package that contains the endpoint structs of your contracts.
We can easily access these endpoint structs (InstantiateMsg, ExecuteMsg, QueryMsg, ...) by adding that package as a dependency to your BOOT crate. 

In order to perform actions on the contract we need to specify these structs so the compiler can type-check our actions. This prevents us from executing a faulty message on a contract and it also handles converting the structs to their json format. The implementation for a CW20 token is shown below. The full file resides [here](boot-plus/src/cw20.rs)

```
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
// Just a type-alias
pub type Cw20<Chain> = CwPlusContract<Chain, ExecuteMsg, InstantiateMsg, QueryMsg, Empty>;

```
You can now perform any action on the cw20 contract and implement custom actions.

```
    let cw20_token = Cw20::new(chain)?;
    let token_info: TokenInfoResponse = cw20_token.query(&Cw20QueryMsg::TokenInfo {}).await?;
```

I recommend reading [the cw20 usage example here](boot-core/examples/cw20.rs)

# Contributing
Feel free to open issues or PRs!
