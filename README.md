# terra-rust-script

Smart contract scripting library to ease smart contract development and deployment.

Terra-rust-script is built on top of [terra-rust-api](https://github.com/PFC-Validator/terra-rust).

The use of this software makes it easier to quickly deploy new contracts. You should use this function responsibly when working on mainnet or testnet as ALL the code you upload to those networks takes up valuable space. Therefore I strongly suggest using [localterra](https://github.com/terra-money/LocalTerra). Setup is very easy and seamlessly supported by using the provided *example.env* file.
## How it works

Usually your contracts workspace will have a package that contains the structs that get filled by a provided JSON through the serde-json package on execution by the CosmWasm VM. 
We can easily access these endpoint structs (InstantiateMsg, ExecuteMsg, QueryMsg, ...) by adding that package as a dependency to the scripting workspace. 

In order to perform actions on the contract we need to specify these structs so the compiler can type-check our actions. This prevents us from executing the message on a wrong contract and it also handles converting the struct to it's json format. This way we prevent sending incorrectly formatted messages! The implementation for a CW20 token is shown below. The full file resides [here](example/cw20.rs)

```
// Wrapper around a ContractInstance that handles address storage and interactions.
pub struct CW20<'a>(ContractInstance<'a>);

// Defining the contract's endpoints.
impl Interface for CW20<'_> {
    type E = Cw20ExecuteMsg;

    type I = InstantiateMsg;

    type Q = Cw20QueryMsg;

    type M = Empty;
}

// Implement the Instance trait so the instance can get accessed
impl Instance for CW20<'_> {
    fn instance(&self) -> &ContractInstance {
        &self.0
    }
}

// A builder function for the struct
impl CW20<'_> {
    pub fn new<'a>(
        name: &'a str,
        sender: &'a Rc<terra_rust_script::sender::Sender<All>>,
        deployment: &'a Deployment,
    ) -> anyhow::Result<CW20<'a>> {
        Ok(CW20(ContractInstance::new(name, sender, deployment)?))
    }

    // Custom functions like send or transfer
    ...
}
```
After implementing these traits you have the ability to use all the functions provided by the WasmInstantiate, WasmExecute, WasmQuery and WasmMigrate traits. This way our script calls are very ergonomic: 

```
    let token_info = cw20_token.query(Cw20QueryMsg::TokenInfo {}).await?;
```


## Usage

1. Create a new dir + workspace to hold the scripts and the generated executable binaries.
   ```
   $ mkdir my_scripts
   $ cd my_scripts
   $ cargo init --bin
   ```
2. Clone the *example.env* file from this repo and rename it to *.env*
3. In order to start using terra-rust-script you need to add the package as a dependency in your Cargo.toml file. (TODO: upload to crates.io)
    ```
    [dependencies]
    terra-rust-script = {git = "https://github.com/CyberHoward/terra-rust-script", tag = "v1.0.0"}
    ```

I prefer to lay out my dir as shown below but anything goes as file paths are set in the .env file.

```
my_scripts/
├─ resources/
│  ├─ // JSONs that store the contract information
├─ src/
│  ├─ bin/
│  │  ├─ // Actual scripts
│  ├─ helpers/
│  │  ├─ // Useful helper functions
│  ├─ contracts/
│  │  ├─ // Contract definitions / custom functions
│  ├─ lib.rs
├─ .env
├─ Cargo.toml
```

### .env file
**Warning: The current version of this software requires you to insert the mnemonic of your wallet in order to sign messages. This is a security risk! I do not take responsibility over lost/stolen funds.**

- **DEPLOYMENT**: Name of the group of contracts you want to address. "mainnet" or "testnet" will automatically use the other configs that apply. Any other name will make the executable default to use localterra. 
- **RUST_LOG**: Level of terminal logging
- **ADDRESS_JSON**: Path to JSON file that stores contract addresses and code-id per group. 
- **WASM_DIR**: Path to dir that stores the .wasm binaries and compile hashes. Used for uploading/verifying.