use std::collections::HashMap;

use cw_orch_core::environment::{ChainInfoOwned, ChainKind, NetworkInfoOwned};
use serde::{Deserialize, Serialize};

use crate::env::default_state_folder;

pub(crate) fn load(chain_id: &str) -> Option<ChainInfoSerde> {
    let mut state_folder = default_state_folder().ok()?;
    state_folder.push("networks.json");
    let file = std::fs::File::open(state_folder).ok()?;

    let mut network_config =
        serde_json::from_reader::<_, HashMap<String, ChainInfoSerde>>(&file).ok()?;
    network_config.remove(chain_id)
}

impl ChainInfoSerde {
    pub fn apply_to(self, mut chain: ChainInfoOwned) -> ChainInfoOwned {
        let ChainInfoSerde {
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
        } = self;

        if !chain_id.is_empty() {
            chain.chain_id = chain_id;
        }
        if !gas_denom.is_empty() {
            chain.gas_denom = gas_denom;
        }
        if !gas_price.is_nan() {
            chain.gas_price = gas_price;
        }
        if !grpc_urls.is_empty() {
            chain.grpc_urls = grpc_urls;
        }
        if let Some(lcd_url) = lcd_url {
            chain.lcd_url = Some(lcd_url);
        }
        if let Some(fcd_url) = fcd_url {
            chain.fcd_url = Some(fcd_url);
        }
        if !chain_name.is_empty() {
            chain.network_info.chain_name = chain_name;
        }
        if !pub_address_prefix.is_empty() {
            chain.network_info.pub_address_prefix = pub_address_prefix;
        }
        if coin_type != 118 {
            chain.network_info.coin_type = coin_type;
        }
        if kind != ChainKindSerde::Unspecified {
            chain.kind = kind.into();
        }
        chain
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub(crate) struct ChainInfoSerde {
    /// Identifier for the network ex. phoenix-2, pisco-1
    pub chain_id: String,
    /// Max gas and denom info
    pub gas_denom: String,
    /// gas price
    pub gas_price: f64,
    /// gRPC urls, used to attempt connection
    pub grpc_urls: Vec<String>,
    /// Optional urls for custom functionality
    pub lcd_url: Option<String>,
    /// Optional urls for custom functionality
    pub fcd_url: Option<String>,
    /// Underlying network details (coin type, address prefix, etc)
    pub network_info: NetworkInfoOwned,
    /// Chain kind, (local, testnet, mainnet)
    pub kind: ChainKindSerde,
}

/// Kind of chain (local, testnet, mainnet)
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChainKindSerde {
    /// A local chain, used for development
    Local,
    /// A mainnet chain
    Mainnet,
    /// A testnet chain
    Testnet,
    #[default]
    /// Unspecified chain kind
    Unspecified,
}

impl Into<ChainKind> for ChainKindSerde {
    fn into(self) -> ChainKind {
        match self {
            ChainKindSerde::Local => ChainKind::Local,
            ChainKindSerde::Mainnet => ChainKind::Mainnet,
            ChainKindSerde::Testnet => ChainKind::Testnet,
            // Shouldn't be reachable
            ChainKindSerde::Unspecified => ChainKind::Local,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{
        networks::{JUNO_1, NEUTRON_1},
        Daemon,
    };
    use cw_orch::{
        environment::{ChainKind, NetworkInfoOwned},
        prelude::{ChainInfo, ChainInfoOwned},
    };

    #[test]
    #[ignore = "This test is for testing config in CI"]
    fn existing_network_config() {
        let chain_info = load(JUNO_1.chain_id)
            .unwrap()
            .apply_to(ChainInfoOwned::from(NEUTRON_1));
        let expected_chain_info = ChainInfoOwned {
            chain_id: "joono-1".to_owned(),
            gas_denom: "gas_denom".to_owned(),
            gas_price: 64f64,
            grpc_urls: vec!["http://juno-grpc.com:123".to_owned()],
            lcd_url: Some("http://juno-lcd.com:321".to_owned()),
            fcd_url: Some("http://juno-fcd.com:234".to_owned()),
            network_info: NetworkInfoOwned {
                chain_name: "joono".to_owned(),
                pub_address_prefix: "joo".to_owned(),
                coin_type: 42,
            },
            kind: cw_orch::environment::ChainKind::Local,
        };
        assert_eq!(chain_info, expected_chain_info);
        // Not testing daemon since we don't have working grpc here
    }

    #[test]
    #[serial_test::serial]
    #[ignore = "This test is for testing config in CI"]
    fn existing_network_partial_config() {
        let chain_info = load(NEUTRON_1.chain_id)
            .unwrap()
            .apply_to(ChainInfoOwned::from(NEUTRON_1));
        let expected_chain_info = ChainInfoOwned {
            gas_price: 1.23f64,
            ..NEUTRON_1.into()
        };
        assert_eq!(chain_info, expected_chain_info);

        let querier_daemon = Daemon::builder(NEUTRON_1).build_sender(()).unwrap();
        assert_eq!(querier_daemon.chain_info().clone(), expected_chain_info);
    }

    #[test]
    #[serial_test::serial]
    #[ignore = "This test is for testing config in CI"]
    fn missing_network_full_config() {
        let chain_info = load("abstr-1").unwrap();
        let expected_chain_info = ChainInfoSerde {
            kind: ChainKindSerde::Mainnet,
            chain_id: "juno-1".to_owned(),
            gas_denom: "ujuno".to_owned(),
            gas_price: 0.0750,
            grpc_urls: vec!["http://juno-grpc.polkachu.com:12690".to_owned()],
            network_info: NetworkInfoOwned {
                chain_name: "juno".to_owned(),
                pub_address_prefix: "juno".to_owned(),
                coin_type: 118u32,
            },
            lcd_url: None,
            fcd_url: None,
        };
        assert_eq!(chain_info, expected_chain_info);

        // It's an actual juno config so we will be able to load it without any issues
        let expected_chain_info = ChainInfoOwned {
            kind: ChainKind::Mainnet,
            chain_id: "juno-1".to_owned(),
            gas_denom: "ujuno".to_owned(),
            gas_price: 0.0750,
            grpc_urls: vec!["http://juno-grpc.polkachu.com:12690".to_owned()],
            network_info: NetworkInfoOwned {
                chain_name: "juno".to_owned(),
                pub_address_prefix: "juno".to_owned(),
                coin_type: 118u32,
            },
            lcd_url: None,
            fcd_url: None,
        };
        let daemon_sender = Daemon::builder(ChainInfo::config("abstr-1"))
            .build_sender(())
            .unwrap();
        assert_eq!(daemon_sender.chain_info().clone(), expected_chain_info);
    }
}
