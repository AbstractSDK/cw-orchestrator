use crate::core::HermesRelayer;
use cosmwasm_std::IbcOrder;
use cw_orch_interchain_core::env::ChainId;
use cw_orch_interchain_daemon::ChannelCreator;
use cw_orch_interchain_daemon::InterchainDaemonError;
use ibc_relayer::chain::requests::{IncludeProof, QueryClientStateRequest, QueryConnectionRequest};
use ibc_relayer::chain::{handle::ChainHandle, requests::QueryHeight};
use ibc_relayer::channel::Channel;
use ibc_relayer::connection::Connection;
use ibc_relayer::foreign_client::ForeignClient;
use ibc_relayer_cli::cli_utils::spawn_chain_runtime;
use ibc_relayer_types::core::ics03_connection::connection::IdentifiedConnectionEnd;
use ibc_relayer_types::core::ics04_channel::channel::Ordering;
use ibc_relayer_types::core::ics24_host::identifier::{self};

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
        let src_connection = self
            .connection_ids
            .get(&(src_chain.to_string(), dst_chain.to_string()))
            .unwrap();

        let config = self.duplex_config(src_chain, dst_chain);

        // Validate & spawn runtime for side a.
        let chain_a =
            spawn_chain_runtime(&config, &identifier::ChainId::from_string(src_chain)).unwrap();

        self.add_key(&chain_a);
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
        self.add_key(&chain_b);

        // Create the foreign client handles.
        let client_a =
            ForeignClient::find(chain_b.clone(), chain_a.clone(), conn_end.client_id()).unwrap();

        let client_b =
            ForeignClient::find(chain_a, chain_b, conn_end.counterparty().client_id()).unwrap();

        let identified_end =
            IdentifiedConnectionEnd::new(src_connection.parse().unwrap(), conn_end);

        let connection = Connection::find(client_a, client_b, &identified_end).unwrap();

        Channel::new(
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
        unimplemented!("
            The Hermes Relayer is a channel creator as well as an Interchain env. 
            You don't need to use this function, you can simply await packets directly on this structure"
        )
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
