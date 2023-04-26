use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::service::ContainerSummary;
use bollard::Docker;
use futures_util::StreamExt;
use tokio::runtime::Runtime;

use crate::{ContractInstance, Daemon};

pub const HERMES_ID: &str = "hermes";

pub struct Hermes {
    pub container: ContainerSummary,
    pub channels: String,
}

impl Hermes {
    pub fn new(container: ContainerSummary) -> Self {
        Self {
            container,
            channels: String::new(),
        }
    }

    /// Execute a command in the hermes container
    pub fn exec_command(&self, runtime: &Runtime, command: Vec<&str>) {
        let docker = Docker::connect_with_local_defaults().unwrap();

        let create_exec_options = CreateExecOptions {
            cmd: Some(command),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            ..Default::default()
        };

        runtime.block_on(async {
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
                if let Ok(a) = a {
                    println!("{}", &a);
                } else {
                    panic!("expected attached exec, got detached");
                };
            }
        })
    }

    // hermes create channel --channel-version simple-ica-v2 --a-chain juno-1 --b-chain osmosis-2 --a-port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --b-port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --new-client-connection
    /// Create an IBC channel between two contracts with an existing client.
    pub fn create_channel(
        &self,
        runtime: &Runtime,
        connection: &str,
        channel_version: &str,
        contract_a: &dyn ContractInstance<Daemon>,
        contract_b: &dyn ContractInstance<Daemon>,
    ) {
        let port_a = contract_port(contract_a);
        let port_b = contract_port(contract_b);

        let command = [
            "hermes",
            "create",
            "channel",
            "--channel-version",
            channel_version,
            "--a-connection",
            connection,
            "--a-chain",
            &contract_a.get_chain().state.chain_id,
            // "--b-chain",
            // &contract_b.get_chain().state.id,
            "--a-port",
            &port_a,
            "--b-port",
            &port_b,
            "--yes",
        ]
        .to_vec();

        self.exec_command(runtime, command)
    }

    /// Create an IBC channel between two contracts with an existing client.
    pub fn start(&self, runtime: &Runtime) {
        let command = ["hermes", "start", "--full-scan"].to_vec();

        self.exec_command(runtime, command)
    }
}

/// format the port for a contract
fn contract_port(contract: &dyn ContractInstance<Daemon>) -> String {
    format!("wasm.{}", contract.addr_str().unwrap())
}
