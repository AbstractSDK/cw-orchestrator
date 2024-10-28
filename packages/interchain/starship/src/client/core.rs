//! Interactions with docker using bollard
use std::fmt::Debug;

use async_std::task::block_on;
use cosmwasm_std::IbcOrder;
use ibc_chain_registry::chain::ChainData;
use kube::runtime::reflector::Lookup;
use serde_json::Value;
use tokio::io::AsyncReadExt;
use url::Url;

use crate::client::StarshipClientError;

use super::registry::Registry;
use super::StarshipClientResult;

// const CHAIN_REGISTRY: &str = "http://localhost:8081/chains";
// const IBC_REGISTRY: &str = "http://localhost:8081/ibc";
const LOCALHOST: &str = "http://localhost";
const DEFAULT_REST: &str = "8081";

// https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
const EXEC_SUCCESS_STATUS: &str = "Success";

/// Represents a set of locally running blockchain nodes and a Hermes relayer.
#[derive(Clone)]
pub struct StarshipClient {
    // Where starship is hosted, uses localhost:8081 by default.
    url: Url,
    /// Daemons indexable by network id, i.e. "juno-1", "osmosis-2", ...
    // chain_config: HashMap<NetworkId, ChainData>,
    pub chains: Vec<ChainData>,
    /// Starship config
    pub starship_config: Option<yaml_rust2::Yaml>,
    kube_client: kube::Client,
}

// kube::Client doesn't implement debug unfortunately
impl Debug for StarshipClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // No debug for kube::Client
        f.debug_struct("StarshipClient")
            .field("url", &self.url)
            .field("chains", &self.chains)
            .field("starship_config", &self.starship_config)
            .finish()
    }
}

impl StarshipClient {
    /// Create a Starship object from the localhost chain registry.
    pub fn new(
        url: Option<&str>,
        starship_config: Option<yaml_rust2::Yaml>,
    ) -> StarshipClientResult<Self> {
        let starship = block_on(Self::new_async(url, starship_config))?;
        Ok(starship)
    }

    /// Builds a new `Starship` instance from the hosted chain registry.
    pub async fn new_async(
        url: Option<&str>,
        starship_config: Option<yaml_rust2::Yaml>,
    ) -> StarshipClientResult<Self> {
        let kube_client = kube::Client::try_default().await?;
        let registry_rest = starship_config
            .as_ref()
            .map(|yaml| {
                yaml["registry"]["ports"]["rest"]
                    .as_i64()
                    .expect("Starship registry port should be a number")
                    .to_string()
            })
            .unwrap_or(DEFAULT_REST.to_string());
        let url: url::Url = url
            .map(|u| u.to_string())
            .unwrap_or_else(|| format!("{LOCALHOST}:{registry_rest}"))
            .parse()?;
        let registry = Registry::new(url.clone()).await;

        // Fetch all chain data from the chain registry
        let chains = registry.chain_data().await?;

        // get all the ibc data:
        Ok(Self {
            url,
            chains,
            starship_config,
            kube_client,
        })
    }

    /// Get the `Registry` object for this `Starship` instance.
    pub async fn registry(&self) -> Registry {
        Registry::new(self.url.clone()).await
    }

    async fn find_hermes_pod(
        &self,
        chain_id_a: &str,
        chain_id_b: &str,
    ) -> StarshipClientResult<String> {
        // Lucky if we can just parse starship config
        if let Some(config) = &self.starship_config {
            // Finding relayer with those 2 chains
            for relayer in config["relayers"].clone() {
                let chains = relayer["chains"]
                    .as_vec()
                    .expect("Missing chains in relayer config");
                let chains = chains
                    .iter()
                    .map(|chain| chain.as_str().unwrap())
                    .collect::<Vec<_>>();
                if chains.contains(&chain_id_a) && chains.contains(&chain_id_b) {
                    let relayer_name = relayer["name"].as_str().unwrap();
                    // Most likely `hermes`
                    let relayer_type = relayer["type"].as_str().unwrap();
                    let relayer_name = format!("{relayer_type}-{relayer_name}");
                    return Ok(relayer_name);
                }
            }
        } else {
            // find an hermes pod with these ids otherwise
            let pods: kube::Api<k8s_openapi::api::core::v1::Pod> =
                kube::Api::default_namespaced(self.kube_client.clone());
            let hermes_pods = pods
                .list(&kube::api::ListParams::default().labels("app.kubernetes.io/type=hermes"))
                .await?;
            for pod in hermes_pods {
                let ap = kube::api::AttachParams::default().container("relayer");
                let pod_name = pod.name().unwrap();
                let mut process = pods
                    .exec(
                        &pod_name,
                        // get chains
                        ["curl", "-s", "-X", "GET", "http://127.0.0.1:3000/chains"],
                        &ap,
                    )
                    .await?;
                let status = process
                    .take_status()
                    .unwrap()
                    .await
                    .unwrap()
                    .status
                    .unwrap();
                if status != EXEC_SUCCESS_STATUS {
                    // Can't get status of kubernetes exec, meaning we can't get correct hermes
                    return Err(StarshipClientError::HermesNotFound);
                }
                let mut async_stdout = process.stdout().unwrap();

                let mut dst = vec![];
                async_stdout.read_to_end(&mut dst).await?;
                let val: Value =
                    serde_json::from_slice(&dst).expect("curl output should be json formatted");
                let chains = val["result"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|chain| chain.as_str().unwrap())
                    .collect::<Vec<_>>();
                if chains.contains(&chain_id_a) && chains.contains(&chain_id_b) {
                    return Ok(pod_name.to_string());
                }
            }
        }
        // Not found
        Err(StarshipClientError::HermesNotFound)
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
        let pods: kube::Api<k8s_openapi::api::core::v1::Pod> =
            kube::Api::default_namespaced(self.kube_client.clone());
        let ap = kube::api::AttachParams::default()
            .container("relayer")
            // hermes channel create expect tty
            .tty(true)
            // Can't attach stderr to tty
            .stderr(false);
        let mut attached_process = pods.exec(&pod_id, command, &ap).await?;
        log::debug!("Waiting for {chain_id_a}-{chain_id_b} via {src_connection_id} open");
        let status = attached_process.take_status().unwrap().await.unwrap();

        // Make sure we succeeded
        if status.status.clone().unwrap() != EXEC_SUCCESS_STATUS {
            log::debug!("{status:?}");
            return Err(StarshipClientError::ChannelCreationFailure(
                chain_id_a.to_owned(),
                chain_id_b.to_owned(),
                status.reason.unwrap_or_default(),
            ));
        }
        attached_process.join().await.unwrap();

        log::debug!("{chain_id_a}-{chain_id_b} via {src_connection_id} created");
        Ok(src_connection_id.to_string())
    }
}
