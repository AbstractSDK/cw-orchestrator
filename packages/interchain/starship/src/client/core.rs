//! Interactions with docker using bollard
use cosmwasm_std::IbcOrder;
use ibc_chain_registry::chain::ChainData;

use tokio::process::Command;
use tokio::runtime::Handle;
use url::Url;

use super::registry::Registry;
use super::StarshipClientResult;

// const CHAIN_REGISTRY: &str = "http://localhost:8081/chains";
// const IBC_REGISTRY: &str = "http://localhost:8081/ibc";
const LOCALHOST: &str = "http://localhost:8081";

// TODO, this needs to come from the localhost as well
const TEMP_HERMES_RELAYER_NAME: &str = "hermes-osmo-juno";

/// Represents a set of locally running blockchain nodes and a Hermes relayer.
#[derive(Debug, Clone)]
pub struct StarshipClient {
    // Where starship is hosted, uses localhost:8081 by default.
    url: Url,
    /// Daemons indexable by network id, i.e. "juno-1", "osmosis-2", ...
    // chain_config: HashMap<NetworkId, ChainData>,
    pub chains: Vec<ChainData>,
}

impl StarshipClient {
    /// Create a Starship object from the localhost chain registry.
    pub fn new(rt: Handle, url: Option<&str>) -> StarshipClientResult<Self> {
        let starship = rt.block_on(Self::new_async(url))?;
        Ok(starship)
    }

    /// Builds a new `Starship` instance from the hosted chain registry.
    pub async fn new_async(url: Option<&str>) -> StarshipClientResult<Self> {
        let url: url::Url = url
            .map(|u| u.to_string())
            .unwrap_or_else(|| LOCALHOST.to_string())
            .parse()?;

        let registry = Registry::new(url.clone()).await;

        // Fetch all chain data from the chain registry
        let chains = registry.chain_data().await?;

        // get all the ibc data:
        Ok(Self { url, chains })
    }

    /// Get the `Registry` object for this `Starship` instance.
    pub async fn registry(&self) -> Registry {
        Registry::new(self.url.clone()).await
    }

    async fn find_hermes_pod(
        &self,
        _chain_id_a: &str,
        _chain_id_b: &str,
    ) -> StarshipClientResult<String> {
        // find an hermes pod with these ids
        let relayer_name = TEMP_HERMES_RELAYER_NAME.to_string();

        // execute on the pod
        let pod_id_out = Command::new("kubectl")
            .args(["get", "pods", "--no-headers"])
            .arg(format!("-lapp.kubernetes.io/name={}", relayer_name))
            .output()
            .await
            .unwrap();

        let pod_id_output = String::from_utf8(pod_id_out.stdout).unwrap();

        let pod_id = pod_id_output.split_whitespace().next().unwrap();
        println!("pod_out: {:?}", pod_id);

        Ok(pod_id.to_string())
    }

    /// Triggers channel creation with the relayer registered between the 2 chains
    pub async fn create_channel(
        &self,
        chain_id_a: &str,
        chain_id_b: &str,
        port_a: &str,
        port_b: &str,
        channel_version: &str,
        order: Option<IbcOrder>,
    ) -> StarshipClientResult<String> {
        let pod_id = self.find_hermes_pod(chain_id_a, chain_id_b).await?;

        // get the ibc channel between the two chains
        let path = self
            .registry()
            .await
            .ibc_path(chain_id_a, chain_id_b)
            .await?;

        let src_connection_id = path.chain_1.connection_id.as_str();

        // create channel by executing on this pod
        let mut command = [
            "hermes",
            "create",
            "channel",
            "--channel-version",
            channel_version,
            "--a-connection",
            src_connection_id,
            "--a-chain",
            chain_id_a,
            // "--b-chain",
            // &contract_b.get_chain().state.id,
            "--a-port",
            port_a,
            "--b-port",
            port_b,
        ]
        .to_vec();

        if let Some(order) = order {
            let order_string = match order {
                IbcOrder::Unordered => "unordered",
                IbcOrder::Ordered => "ordered",
            };
            command.push("--order");
            command.push(order_string);
        }

        // now execute on the pod
        let _execute_channel_create = Command::new("kubectl")
            .arg("exec")
            .arg(&pod_id)
            .arg("--")
            .args(command)
            .output()
            .await
            .unwrap();

        Ok(src_connection_id.to_string())
    }
}
