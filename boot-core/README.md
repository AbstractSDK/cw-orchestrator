# CosmWasm BOOT
A Rust gRPC and mock multi-test scripting library. 

## Environment variables

> **The env variable file might contain sensitive information (like a mnemonic phrase). We not responsible for your security practices.**  
> A new daemon store file scaffold will be automatically generated when you try to run a script for the first time.

| Entry | Description |
| ----------- | ----------- |
| DAEMON_STATE_PATH | File that stores network info and state |
| RUST_LOG | Debug level for logging |
| WASM_DIR   | Directory path that holds optimized `.wasm` builds |
| LOCAL_MNEMONIC   | Mnemonic used when `NETWORK="local"` |
| TEST_MNEMONIC   | Mnemonic used when `NETWORK="testnet"` |
| MAIN_MNEMONIC   | Mnemonic used when `NETWORK="mainnet"` |

## Usage

Since boot core allows you to write environment-generic functions to call your contracts, you can use these functions in two different places. 
Therefore our crate will have both a library and binary component.   

