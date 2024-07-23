use cw_orch::{
    environment::{ChainKind, NetworkInfoOwned},
    prelude::{ChainInfo, ChainInfoOwned},
};
use cw_orch_daemon::{
    networks::{JUNO_1, NEUTRON_1},
    Daemon,
};

#[test]
#[ignore = "This test is for testing config in CI"]
fn existing_network_config() {
    let network_config = cw_orch_daemon::NetworkConfig::load().unwrap();

    let chain_info = network_config.update_chain_info(JUNO_1.into());
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
    let network_config = cw_orch_daemon::NetworkConfig::load().unwrap();

    let chain_info_owned: ChainInfoOwned = NEUTRON_1.into();
    let chain_info = network_config.update_chain_info(NEUTRON_1.into());
    let expected_chain_info = ChainInfoOwned {
        gas_price: 1.23f64,
        ..chain_info_owned
    };
    assert_eq!(chain_info, expected_chain_info);

    let querier_daemon = Daemon::builder(NEUTRON_1).build_sender(()).unwrap();
    assert_eq!(querier_daemon.chain_info().clone(), expected_chain_info);
}

#[test]
#[serial_test::serial]
#[ignore = "This test is for testing config in CI"]
fn missing_network_full_config() {
    let network_config = cw_orch_daemon::NetworkConfig::load().unwrap();

    let chain_info = network_config.update_chain_info(ChainInfoOwned::config("abstr-1".to_owned()));
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

/*** Mock networks file
{
  "juno-1": {
    "chain_id": "joono-1",
    "gas_denom": "gas_denom",
    "gas_price": 64.0,
    "grpc_urls": ["http://juno-grpc.com:123"],
    "lcd_url": "http://juno-lcd.com:321",
    "fcd_url": "http://juno-fcd.com:234",
    "network_info": {
      "chain_name": "joono",
      "pub_address_prefix": "joo",
      "coin_type": 42
    },
    "kind": "local"
  },
  "neutron-1": {
    "gas_price": 1.23
  },
  "abstr-1": {
    "kind": "mainnet",
    "chain_id": "juno-1",
    "gas_denom": "ujuno",
    "gas_price": 0.075,
    "grpc_urls": ["http://juno-grpc.polkachu.com:12690"],
    "network_info": {
      "chain_name": "juno",
      "pub_address_prefix": "juno",
      "coin_type": 118
    }
  }
}
***/
