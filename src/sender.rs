use secp256k1::{Context, Secp256k1, Signing};
use serde_json::{from_reader, json};
use std::{env, fs::File};
use terra_rust_api::{errors::TerraRustAPIError, GasOptions, PrivateKey, Terra};

use crate::error::TerraRustScriptError;

pub struct Sender<C: Signing + Context> {
    pub terra: Terra,
    pub private_key: PrivateKey,
    pub secp: Secp256k1<C>,
}

impl<C: Signing + Context> Sender<C> {
    pub fn pub_addr(&self) -> Result<String, TerraRustAPIError> {
        self.private_key.public_key(&self.secp).account()
    }
    pub fn new(
        config: &GroupConfig,
        secp: Secp256k1<C>,
    ) -> Result<Sender<C>, TerraRustScriptError> {
        // NETWORK_MNEMONIC_GROUP
        let mut composite_name = config.network_config.network.mnemonic_name().to_string();
        composite_name.push_str("_");
        composite_name.push_str(&config.name.to_ascii_uppercase());

        let p_key: PrivateKey;

        // use group mnemonic if specified, elso use default network mnemonic
        if let Some(mnemonic) = env::var_os(&composite_name) {
            p_key = PrivateKey::from_words(&secp, mnemonic.to_str().unwrap(), 0, 0)?;
        } else {
            log::debug!("{}", config.network_config.network.mnemonic_name());
            let mnemonic = env::var(config.network_config.network.mnemonic_name())?;
            p_key = PrivateKey::from_words(&secp, &mnemonic, 0, 0)?;
        }

        Ok(Sender {
            terra: Terra::lcd_client(
                config.network_config.lcd_url.clone(),
                config.network_config.chain_id.clone(),
                &config.network_config.gas_opts,
                None,
            ),
            private_key: p_key,
            secp,
        })
    }
}
#[derive(Clone, Debug)]

pub enum Network {
    LocalTerra,
    Mainnet,
    Testnet,
}

impl Network {
    async fn config(&self, client: reqwest::Client, denom: &str) -> anyhow::Result<NetworkConfig> {
        let conf = match self {
            Network::LocalTerra => (
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
        let gas_opts = GasOptions::create_with_fcd(&client, &conf.1, denom, 1.3f64).await?;

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
            Network::LocalTerra => "LOCAL_MNEMONIC",
            Network::Mainnet => "MAIN_MNEMONIC",
            Network::Testnet => "TEST_MNEMONIC",
        }
    }

    pub fn multisig_name(&self) -> &str {
        match *self {
            Network::LocalTerra => "LOCAL_MULTISIG",
            Network::Mainnet => "MAIN_MULTISIG",
            Network::Testnet => "TEST_MULTISIG",
        }
    }
}
#[derive(Clone, Debug)]
pub struct GroupConfig {
    pub network_config: NetworkConfig,
    pub name: String,
    pub file_path: String,
    pub proposal: bool,
}

impl GroupConfig {
    pub async fn new<C: secp256k1::Signing + secp256k1::Context>(
        network: Network,
        name: String,
        client: reqwest::Client,
        denom: &str,
        file_path: String,
        proposal: bool,
        _secp: &Secp256k1<C>,
    ) -> anyhow::Result<GroupConfig> {
        check_group_existance(&name, &file_path)?;

        Ok(GroupConfig {
            network_config: network.config(client, denom).await?,
            name,
            file_path,
            proposal,
        })
    }

    pub fn get_contract_address(
        &self,
        contract_name: &str,
    ) -> Result<String, TerraRustScriptError> {
        let file = File::open(&self.file_path)
            .expect(&format!("file should be present at {}", self.file_path));
        let json: serde_json::Value = from_reader(file)?;
        let maybe_address = json[self.name.clone()][contract_name].get("addr");
        match maybe_address {
            Some(addr) => {
                log::debug!("contract: {} addr: {}", self.name, addr);
                return Ok(addr.as_str().unwrap().into());
            }
            None => {
                return Err(TerraRustScriptError::AddrNotInFile(
                    contract_name.to_owned(),
                ))
            }
        }
    }

    pub fn get_contract_code_id(&self, contract_name: &str) -> Result<u64, TerraRustScriptError> {
        let file = File::open(&self.file_path)
            .expect(&format!("file should be present at {}", self.file_path));
        let json: serde_json::Value = from_reader(file).unwrap();
        let maybe_code_id = json[self.name.clone()][contract_name].get("code_id");
        match maybe_code_id {
            Some(code_id) => {
                log::debug!("contract: {} code_id: {}", self.name, code_id);
                return Ok(code_id.as_u64().unwrap());
            }
            None => {
                return Err(TerraRustScriptError::AddrNotInFile(
                    contract_name.to_owned(),
                ))
            }
        }
    }
}

fn check_group_existance(name: &String, file_path: &String) -> anyhow::Result<()> {
    let file = File::open(file_path).expect(&format!("file should be present at {}", file_path));
    let mut cfg: serde_json::Value = from_reader(file).unwrap();
    let maybe_group = cfg.get(name);
    match maybe_group {
        Some(_) => {
            return Ok(());
        }
        None => {
            cfg[name] = json!({});
            serde_json::to_writer_pretty(File::create(file_path)?, &cfg)?;
            return Ok(());
        }
    }
}
#[derive(Clone, Debug)]
pub struct NetworkConfig {
    pub network: Network,
    pub lcd_url: String,
    pub fcd_url: String,
    pub chain_id: String,
    pub gas_opts: GasOptions,
}
