use crate::daemon::Daemon;
use crate::interface_traits::ContractInstance;
use crate::state::ChainState;
use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::service::ContainerSummary;
use bollard::Docker;
use futures_util::StreamExt;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

use super::interchain_env::contract_port;

pub const HERMES_ID: &str = "hermes";

/// Structure to interact with a local hermes docker container and simulate
/// This is specifically designed to interact with the following repo :
/// https://github.com/AbstractSDK/interchaintest/
/// Usually, for testing, one would setup a test infrastructure using :
/// ```bash
///     go test examples/ibc/cw_ibc_test.go
/// ```
pub struct Hermes {
    /// Instance of the docker container that contains the Hermes relayer infra
    pub container: ContainerSummary,
}

impl Hermes {
    /// Creates an hermes instance from a docker container information
    pub fn new(container: ContainerSummary) -> Self {
        Self { container }
    }

    /// Execute a command in the hermes container
    pub async fn exec_command(&self, command: Vec<&str>) {
        let docker = Docker::connect_with_local_defaults().unwrap();

        let create_exec_options = CreateExecOptions {
            cmd: Some(command),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            ..Default::default()
        };

        {
            let create_exec_response = docker
                .create_exec(self.container.id.as_ref().unwrap(), create_exec_options)
                .await
                .expect("failed to create exec");

            let exec_id = create_exec_response.id;

            let StartExecResults::Attached { mut output, input: _ } = docker
        .start_exec(&exec_id, Some(StartExecOptions { detach: false, ..Default::default() }))
        .await
        .expect("failed to start exec")
    else {
        panic!("expected attached exec, got detached");
    };
            while let Some(a) = output.next().await {
                if a.is_ok() {
                } else {
                    panic!("expected attached exec, got detached");
                };
            }
        }
    }

    // hermes create channel --channel-version simple-ica-v2 --a-chain juno-1 --b-chain osmosis-2 --a-port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --b-port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --new-client-connection
    /// Create an IBC channel between two contracts with an existing client.
    pub async fn create_channel(
        &self,
        connection_a: &str,
        channel_version: &str,
        contract_a: &dyn ContractInstance<Daemon>,
        contract_b: &dyn ContractInstance<Daemon>,
    ) {
        let port_a = contract_port(contract_a);
        let port_b = contract_port(contract_b);

        let chain_id = &contract_a
            .get_chain()
            .state()
            .chain_data
            .chain_id
            .to_string();

        self.create_channel_raw(connection_a, channel_version, chain_id, port_a, port_b)
            .await
    }

    /// Creates a channel on the Hermes container between two ports with an existing client
    pub async fn create_channel_raw(
        &self,
        connection_a: &str,
        channel_version: &str,
        chain_id_a: &str,
        port_a: PortId,
        port_b: PortId,
    ) {
        let command = [
            "hermes",
            "create",
            "channel",
            "--channel-version",
            channel_version,
            "--a-connection",
            connection_a,
            "--a-chain",
            chain_id_a,
            // "--b-chain",
            // &contract_b.get_chain().state.id,
            "--a-port",
            port_a.as_str(),
            "--b-port",
            port_b.as_str(),
            "--yes",
        ]
        .to_vec();

        self.exec_command(command).await
    }

    /// Create an IBC channel between two contracts with an existing client.
    pub async fn start(&self) {
        let command = ["hermes", "start", "--full-scan"].to_vec();

        self.exec_command(command).await
    }
}
