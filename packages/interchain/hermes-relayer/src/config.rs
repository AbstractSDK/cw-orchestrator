use std::time::Duration;

use cw_orch_core::environment::ChainInfoOwned;
use ibc_relayer::chain::cosmos::config::CosmosSdkConfig;
use ibc_relayer::config::gas_multiplier::GasMultiplier;
use ibc_relayer::config::types::{MaxMsgNum, MaxTxSize, Memo};
use ibc_relayer::config::{AddressType, ChainConfig, EventSourceMode, GasPrice, RefreshRate};
use ibc_relayer::keyring::Store;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use ibc_relayer_types::core::ics24_host::identifier::{self};

pub const KEY_NAME: &str = "relayer";

pub fn chain_config(
    chain: &str,
    rpc_url: &str,
    chain_data: &ChainInfoOwned,
    is_consumer_chain: bool,
) -> ChainConfig {
    ChainConfig::CosmosSdk(CosmosSdkConfig {
        id: identifier::ChainId::from_string(chain),

        rpc_addr: rpc_url.parse().unwrap(),
        grpc_addr: chain_data.grpc_urls[0].parse().unwrap(),
        event_source: EventSourceMode::Pull {
            interval: Duration::from_secs(4),
            max_retries: 4,
        },
        rpc_timeout: Duration::from_secs(10),
        trusted_node: false,
        account_prefix: chain_data.network_info.pub_address_prefix.to_string(),
        key_name: KEY_NAME.to_string(),
        key_store_type: Store::Memory,
        key_store_folder: None,
        store_prefix: "ibc".to_string(),
        default_gas: Some(100000),
        max_gas: Some(2000000),
        genesis_restart: None,
        gas_adjustment: None,
        gas_multiplier: Some(GasMultiplier::new(1.3).unwrap()),
        fee_granter: None,
        max_msg_num: MaxMsgNum::new(30).unwrap(),
        max_tx_size: MaxTxSize::new(180000).unwrap(),
        max_grpc_decoding_size: 33554432u64.into(),
        clock_drift: Duration::from_secs(5),
        max_block_time: Duration::from_secs(30),
        trusting_period: None,
        ccv_consumer_chain: is_consumer_chain,
        memo_prefix: Memo::new("").unwrap(),
        sequential_batch_tx: false,
        proof_specs: None,
        trust_threshold: TrustThreshold::new(1, 3).unwrap(),
        gas_price: GasPrice::new(chain_data.gas_price, chain_data.gas_denom.to_string()),
        packet_filter: Default::default(),
        address_type: AddressType::Cosmos,
        extension_options: Default::default(),
        query_packets_chunk_size: 10,
        client_refresh_rate: RefreshRate::new(5, 1),
        memo_overwrite: Default::default(),
        dynamic_gas_price: Default::default(),
        compat_mode: Default::default(),
        clear_interval: Default::default(),
        excluded_sequences: Default::default(),
        allow_ccq: true,
    })
}
