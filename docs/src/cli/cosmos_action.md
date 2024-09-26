# Cosmos Action

This command allows to perform an action on cosmos chain, if this action requires signing given [Signer](./keys.md) would be used

## Chain id

For doing any cosmos action chain id is needed, so it have to be provided before selecting any of the subcommand

## Features

### CosmWasm Action

Interact with CosmWasm smart contract

- Store smart contract: `cw-orch-cli action [CHAIN_ID] cw store [WASM_PATH] [SIGNER]`
- Instantiate smart contract:  `cw-orch-cli action [CHAIN_ID] cw instantiate [CODE_ID] [MSG_TYPE] [MSG] [LABEL] [ADMIN] [COINS] [SIGNER]`
- Execute smart contract method: `cw-orch-cli action [CHAIN_ID] cw execute [OPTIONS] [CONTRACT_ADDR] [MSG_TYPE] [MSG] [COINS] [SIGNER]`
- Query smart contract:
  - Smart query: `cw-orch-cli action [CHAIN_ID] cw query smart [OPTIONS] [CONTRACT] [MSG_TYPE] [MSG]`
  - Raw state query: `cw-orch-cli action [CHAIN_ID] cw query raw [OPTIONS] [CONTRACT] [KEY_TYPE] [KEY]`
    - [KEY_TYPE] supports 2 types: `raw`, `base64`(for non-human-readable keys)

### Asset Action

Send or query assets on cosmos chain

- Send native or factory coin: `cw-orch-cli action [CHAIN_ID] asset send-native [OPTIONS] [COINS] [TO_ADDRESS] [SIGNER]`
- Send cw20 coin: `cw-orch-cli action [CHAIN_ID] asset send-cw20 [OPTIONS] [CW20_ADDRESS] [AMOUNT] [TO_ADDRESS] [SIGNER]`
- Query native or factory coin balance: `cw-orch-cli action [CHAIN_ID] asset query-native [OPTIONS] [DENOM] [ADDRESS]`
  - For querying all balances use empty string ("") instead of [DENOM]
- Query cw20 balance: `cw-orch-cli action [CHAIN_ID] asset query-cw20 [OPTIONS] [CW20_ADDRESS] [ADDRESS]`

### CW-Ownable Action

Interact with cw-ownable controller on CosmWasm smart contract

- Propose to transfer contract ownership to another address: `cw-orch-cli action [CHAIN_ID] cw-ownable transfer [OPTIONS] [CONTRACT] [NEW_OWNER] [EXPIRATION] [SIGNER] [NEW_SIGNER]`
  - [EXPIRATION] supports three variants: `never`, `height:{block_height}`, `time:{time_nanos}`
  - If you cannot sign [NEW_SIGNER] use empty string("") instead
- Accept pending ownership: `cw-orch-cli action [CHAIN_ID] cw-ownable accept [OPTIONS] [CONTRACT] [SIGNER]`
- Renounce pending ownership: `cw-orch-cli action [CHAIN_ID] cw-ownable renounce [OPTIONS] [CONTRACT] [SIGNER]`
- Get current ownership: `cw-orch-cli action [CHAIN_ID] cw-ownable get [OPTIONS] [CONTRACT]`
  
#### Arguments reference

- [MSG_TYPE] is a format for provided `[MSG]`, possible values: `json-msg`, `base64-msg`, `file`, `editor`
- [COINS] formatted and parsed same way as `cosmwasm_std::Coins`, for example: "5ujunox,15utestx"
