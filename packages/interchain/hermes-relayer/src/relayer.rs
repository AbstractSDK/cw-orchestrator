use std::collections::HashMap;

use crate::config::{chain_config, KEY_NAME};
use crate::keys::restore_key;
use cosmwasm_std::IbcOrder;
use cw_orch_core::environment::ChainInfoOwned;
use cw_orch_core::environment::ChainState;
use cw_orch_daemon::Daemon;
use cw_orch_interchain_core::env::ChainId;
use cw_orch_interchain_daemon::ChannelCreator;
use cw_orch_interchain_daemon::DaemonInterchainEnv;
use cw_orch_interchain_daemon::InterchainDaemonError;
use cw_orch_interchain_daemon::{IcDaemonResult, Mnemonic};
use ibc_relayer::chain::requests::{IncludeProof, QueryClientStateRequest, QueryConnectionRequest};
use ibc_relayer::chain::{handle::ChainHandle, requests::QueryHeight};
use ibc_relayer::channel::Channel;
use ibc_relayer::config::{Config, RestConfig, TelemetryConfig};
use ibc_relayer::connection::Connection;
use ibc_relayer::foreign_client::ForeignClient;
use ibc_relayer_cli::cli_utils::spawn_chain_runtime;
use ibc_relayer_types::core::ics03_connection::connection::IdentifiedConnectionEnd;
use ibc_relayer_types::core::ics04_channel::channel::Ordering;
use ibc_relayer_types::core::ics24_host::identifier::{self};
use tokio::runtime::Handle;

impl ChannelCreator for HermesRelayer {
    fn create_ibc_channel(
        &self,
        src_chain: ChainId,
        dst_chain: ChainId,
        src_port: &old_ibc_relayer_types::core::ics24_host::identifier::PortId,
        dst_port: &old_ibc_relayer_types::core::ics24_host::identifier::PortId,
        version: &str,
        order: Option<IbcOrder>,
    ) -> Result<String, InterchainDaemonError> {
        // TODO connection should be a parameter
        let src_connection = self
            .connection_ids
            .get(&(src_chain.to_string(), dst_chain.to_string()))
            .unwrap();

        let (src_daemon, src_is_consumer_chain, src_rpc_url) = self.daemons.get(src_chain).unwrap();
        let src_chain_data = &src_daemon.state().chain_data;
        let src_hd_path = src_daemon.wallet().options().hd_index;

        let (dst_daemon, dst_is_consumer_chain, dst_rpc_url) = self.daemons.get(dst_chain).unwrap();
        let dst_chain_data = &dst_daemon.state().chain_data;
        let dst_hd_path = dst_daemon.wallet().options().hd_index;

        let mnemonic = std::env::var("TEST_MNEMONIC").unwrap();

        let config = Config {
            global: ibc_relayer::config::GlobalConfig {
                log_level: ibc_relayer::config::LogLevel::Debug,
            },
            mode: ibc_relayer::config::ModeConfig::default(),
            rest: RestConfig::default(),
            telemetry: TelemetryConfig::default(),
            chains: vec![
                chain_config(
                    src_chain,
                    src_rpc_url,
                    src_chain_data,
                    *src_is_consumer_chain,
                ),
                chain_config(
                    dst_chain,
                    dst_rpc_url,
                    dst_chain_data,
                    *dst_is_consumer_chain,
                ),
            ],
            tracing_server: Default::default(),
        };

        // Validate & spawn runtime for side a.
        let chain_a =
            spawn_chain_runtime(&config, &identifier::ChainId::from_string(src_chain)).unwrap();

        let src_key =
            restore_key(mnemonic.clone(), src_hd_path.unwrap_or(0), src_chain_data).unwrap();
        chain_a.add_key(KEY_NAME.to_string(), src_key).unwrap();

        // Query the connection end.
        let (conn_end, _) = chain_a
            .query_connection(
                QueryConnectionRequest {
                    connection_id: src_connection.parse().unwrap(),
                    height: QueryHeight::Latest,
                },
                IncludeProof::No,
            )
            .unwrap();

        // Query the client state, obtain the identifier of chain b.
        let chain_b = chain_a
            .query_client_state(
                QueryClientStateRequest {
                    client_id: conn_end.client_id().clone(),
                    height: QueryHeight::Latest,
                },
                IncludeProof::No,
            )
            .map(|(cs, _)| cs.chain_id())
            .unwrap();

        // Spawn the runtime for side b.
        let chain_b = spawn_chain_runtime(&config, &chain_b).unwrap();
        let dst_key = restore_key(mnemonic, dst_hd_path.unwrap_or(0), dst_chain_data).unwrap();
        chain_b.add_key(KEY_NAME.to_string(), dst_key).unwrap();

        // Create the foreign client handles.
        let client_a =
            ForeignClient::find(chain_b.clone(), chain_a.clone(), conn_end.client_id()).unwrap();

        let client_b =
            ForeignClient::find(chain_a, chain_b, conn_end.counterparty().client_id()).unwrap();

        let identified_end =
            IdentifiedConnectionEnd::new(src_connection.parse().unwrap(), conn_end);

        let connection = Connection::find(client_a, client_b, &identified_end).unwrap();

        let channel = Channel::new(
            connection,
            cosmwasm_to_hermes_order(order),
            src_port.to_string().parse().unwrap(),
            dst_port.to_string().parse().unwrap(),
            Some(version.to_string().into()),
        )
        .unwrap();

        Ok(src_connection.to_string())
    }

    fn interchain_env(&self) -> cw_orch_interchain_daemon::DaemonInterchainEnv<Self> {
        panic!();
    }
}

fn cosmwasm_to_hermes_order(order: Option<IbcOrder>) -> Ordering {
    match order {
        Some(order) => match order {
            IbcOrder::Unordered => Ordering::Unordered,
            IbcOrder::Ordered => Ordering::Ordered,
        },
        None => Ordering::Unordered,
    }
}
