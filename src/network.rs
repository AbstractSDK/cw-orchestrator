use std::env;

use cosmrs::tx::Gas;

#[derive(Clone, Debug)]
pub struct NetworkConfig {
    pub network: Network,
    pub lcd_url: String,
    pub fcd_url: String,
    pub chain_id: String,
    pub gas_opts: Gas,
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