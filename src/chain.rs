use std::env;

use cosmrs::tx::Gas;

use crate::config::NetworkConfig;

use tonic::transport::Channel;

use crate::error::TerraRustScriptError;

#[derive(Clone, Debug)]
pub struct Chain {
    pub chain_id: String,
    pub pub_addr_prefix: String,
    pub coin_type: u32,
    pub rpc_channel: Channel
}

impl Chain {
    pub async fn new() -> Result<Self, TerraRustScriptError> {
        
        let file = File::open(&self.file_path)
            .expect(&format!("file should be present at {}", self.file_path));
        let json: serde_json::Value = from_reader(file).unwrap();
        let maybe_code_id = json[self.name.clone()][contract_name].get("code_id");
        match maybe_code_id {
            Some(code_id) => Ok(code_id.as_u64().unwrap()),
            None => Err(TerraRustScriptError::AddrNotInFile(
                contract_name.to_owned(),
            )),
        }


        let rpc_channel = Channel::from_shared(grpc_url.clone()).unwrap()
        .connect()
        .await?;

        Ok(Self{
            chain_id,
            coin_type,
            pub_addr_prefix,
            rpc_channel
        })
    }
}

#[derive(Clone, Debug)]
pub enum Network {
    Local,
    Mainnet,
    Testnet,
}

impl Network {
    pub async fn config(&self, client: reqwest::Client, denom: &str) -> anyhow::Result<NetworkConfig> {
        let conf = match self {
            Network::Local => (
                env::var("LTERRA_LCD")?,
                env::var("LTERRA_FCD")?,
                env::var("LTERRA_ID")?,
            ),
            Network::Mainnet => (
                env::var("MAINNET_LCD")?,
                env::var("MAINNET_FCD")?,
                env::var("MAINNET_ID")?,
            ),
            Network::Testnet => (
                env::var("TESTNET_LCD")?,
                env::var("TESTNET_FCD")?,
                env::var("TESTNET_ID")?,
            ),
        };
        
        // Todo: Query value
        let gas_opts: Gas = 94365681.into();

        Ok(NetworkConfig {
            network: self.clone(),
            lcd_url: conf.0,
            fcd_url: conf.1,
            chain_id: conf.2,
            gas_opts,
        })
    }

    pub fn mnemonic_name(&self) -> &str {
        match *self {
            Network::Local => "LOCAL_MNEMONIC",
            Network::Mainnet => "MAIN_MNEMONIC",
            Network::Testnet => "TEST_MNEMONIC",
        }
    }

    pub fn multisig_name(&self) -> &str {
        match *self {
            Network::Local => "LOCAL_MULTISIG",
            Network::Mainnet => "MAIN_MULTISIG",
            Network::Testnet => "TEST_MULTISIG",
        }
    }
}