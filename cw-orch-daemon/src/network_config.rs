use std::collections::HashMap;

use cw_orch_core::environment::ChainInfoOwned;

use crate::env::default_state_folder;

pub(crate) fn load(chain_id: &str) -> Option<ChainInfoOwned> {
    let mut state_folder = default_state_folder().ok()?;
    state_folder.push("networks.json");
    let file = std::fs::File::open(state_folder).ok()?;

    let mut network_config =
        serde_json::from_reader::<_, HashMap<String, ChainInfoOwned>>(&file).ok()?;
    network_config.remove(chain_id)
}

#[cfg(test)]
mod test {
    use super::load;

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
        let chain_info =
            ChainInfoOwned::from(NEUTRON_1).overwrite_with(load(JUNO_1.chain_id).unwrap());
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
        let chain_info =
            ChainInfoOwned::from(NEUTRON_1).overwrite_with(load(NEUTRON_1.chain_id).unwrap());
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
        assert_eq!(chain_info, expected_chain_info);

        // It's an actual juno config so we will be able to load it without any issues
        let daemon_sender = Daemon::builder(ChainInfo::config("abstr-1"))
            .build_sender(())
            .unwrap();
        assert_eq!(daemon_sender.chain_info().clone(), expected_chain_info);
    }
}
