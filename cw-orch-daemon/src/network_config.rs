use std::collections::HashMap;

use cw_orch_core::environment::{ChainInfoOwned, ChainKind, NetworkInfoOwned};
use log::{log, Level};

use crate::env::default_state_folder;

pub struct NetworkConfig(HashMap<String, ChainInfoOwned>);

impl NetworkConfig {
    pub fn load() -> Option<Self> {
        let config_path = match default_state_folder() {
            Ok(mut state_folder) => {
                state_folder.push("networks.json");
                state_folder
            }
            Err(err) => {
                log!(
                    Level::Warn,
                    "Failed to get network config {err}, using default config"
                );
                return None;
            }
        };
        let data = match std::fs::read(config_path) {
            Ok(data) => data,
            Err(e) => {
                log!(
                    Level::Info,
                    "Couldn't read network config file: {e}, using default config"
                );
                return None;
            }
        };

        match serde_json::from_slice(&data) {
            Ok(network_config) => Some(Self(network_config)),
            // Only this is critical
            Err(e) => panic!("Invalid network config: {e}"),
        }
    }

    pub fn update_chain_info(&self, mut chain_info: ChainInfoOwned) -> ChainInfoOwned {
        // If empty config - take full configuration from config file
        if chain_info
            == (ChainInfoOwned {
                chain_id: chain_info.chain_id.clone(),
                ..Default::default()
            })
        {
            return self.config(&chain_info.chain_id);
        }

        let Some(ChainInfoOwned {
            chain_id,
            gas_denom,
            gas_price,
            grpc_urls,
            lcd_url,
            fcd_url,
            network_info:
                NetworkInfoOwned {
                    chain_name,
                    pub_address_prefix,
                    coin_type,
                },
            kind,
        }) = self.0.get(&chain_info.chain_id)
        else {
            return chain_info.clone();
        };

        if !chain_id.is_empty() {
            chain_info.chain_id = chain_id.clone();
        }
        if !gas_denom.is_empty() {
            chain_info.gas_denom = gas_denom.clone();
        }
        if !gas_price.is_nan() {
            chain_info.gas_price = gas_price.clone();
        }
        if !grpc_urls.is_empty() {
            chain_info.grpc_urls = grpc_urls.clone();
        }
        if let Some(lcd_url) = lcd_url {
            chain_info.lcd_url = Some(lcd_url.clone());
        }
        if let Some(fcd_url) = fcd_url {
            chain_info.fcd_url = Some(fcd_url.clone());
        }
        if !chain_name.is_empty() {
            chain_info.network_info.chain_name = chain_name.clone()
        }
        if !pub_address_prefix.is_empty() {
            chain_info.network_info.pub_address_prefix = pub_address_prefix.clone()
        }
        if *coin_type != 118 {
            chain_info.network_info.coin_type = *coin_type;
        }
        if *kind != ChainKind::Unspecified {
            chain_info.kind = kind.clone();
        }
        chain_info
    }

    fn config(&self, chain_id: &str) -> ChainInfoOwned {
        let mut chain_info = self
            .0
            .get(chain_id)
            .expect(&format!("Custom config for {chain_id} not found."))
            .to_owned();

        if chain_info.chain_id.is_empty() {
            chain_info.chain_id = chain_id.to_owned();
        }
        if chain_info.gas_denom.is_empty() {
            panic!("Missing gas_denom in custom config of {chain_id}");
        }
        if chain_info.gas_price.is_nan() {
            panic!("Missing gas_price in custom config of {chain_id}");
        }
        if chain_info.grpc_urls.is_empty() {
            panic!("Missing grpc_urls in custom config of {chain_id}")
        }
        if chain_info.network_info.chain_name.is_empty() {
            panic!("Missing network_info.chain_name in custom config of {chain_id}")
        }
        if chain_info.network_info.pub_address_prefix.is_empty() {
            panic!("Missing network_info.pub_address_prefix in custom config of {chain_id}")
        }
        if chain_info.kind == ChainKind::Unspecified {
            panic!("Missing kind in custom config of {chain_id}")
        }
        chain_info
    }
}
