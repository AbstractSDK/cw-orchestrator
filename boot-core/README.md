# Cosmos Script
A Rust gRPC-based scripting library. 

## Environment variables

> **These env variables might contain sensitive information (like a mnemonic phrase). I am not responsible for your security practices.**  
> A new chain/network scaffold will be automatically generated when you try to run a script with unknown `.env` values.


| Entry | Description |
| ----------- | ----------- |
| CHAIN  | Name of the chain you're targeting (juno, terra, osmosis, ...) |
| NETWORK  | The kind of network you are targeting (local, testnet, mainnet) |
| DEPLOYMENT   | Name of the targeted deployment group |
| RUST_LOG | Debug level for logging |
| WASM_DIR   | Directory path that holds optimized `.wasm` builds |
| STORE | File that stores network info and state |
| LOCAL_MNEMONIC   | Mnemonic used when `NETWORK="local"` |
| TEST_MNEMONIC   | Mnemonic used when `NETWORK="testnet"` |
| MAIN_MNEMONIC   | Mnemonic used when `NETWORK="mainnet"` |
| LOCAL_MULTISIG   | Multisig addr used when `NETWORK="local"` |
| TEST_MULTISIG   | Multisig addr used when `NETWORK="testnet"` |
| MAIN_MULTISIG   | Multisig addr used when `NETWORK="mainnet"` |

## Usage

1. Create a new dir + workspace to hold the scripts and the generated executable binaries.
   ```
   $ mkdir my_scripts
   $ cd my_scripts
   $ cargo init --bin
   ```
2. Clone the [`example.env`](example.env) file to your scripting workspace and rename it to `.env`. Update the values as required and make sure `.env` is included in your `.gitignore` file!  
3. Next, copy the [`default_store.json`](default_store.json) and add whatever chain you want. 
4. In order to start using cosm-script you need to add the package as a dependency in your Cargo.toml file. (TODO: upload to crates.io)
    ```
    [dependencies]
    cosm-script = {git = "", tag = "v1.0.0"}
    ```

Your layout should look something like this

```
my_scripts/
├─ src/
│  ├─ bin/
│  │  ├─ first_stript.rs
│  │  ├─ // My actual scripts
│  ├─ main.rs
├─ default_store.json
├─ .env
├─ Cargo.toml
```

You can then add a custom library, similar to [cw-plus-script](../cw-plus-script/README.md), that holds your custom contract interface definitions.

## Example 
See the [cw-20 example](../cw-plus-script/examples/cw20.rs) for an example on how to use the library. 