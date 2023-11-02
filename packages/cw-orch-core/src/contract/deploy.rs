//! Introduces the Deploy trait only
use anyhow::bail;
use cosmwasm_std::Addr;
use serde_json::from_reader;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::fs::remove_file;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::env::CwOrchEnvVars;
use crate::environment::CwEnv;
use crate::environment::StateInterface;
use crate::CwEnvError;

use super::interface_traits::ContractInstance;

/// Indicates the ability to deploy an application to a mock chain.
///
/// ## Example:
/// ```ignore
/// use cw_orch::{Deploy, CwOrchError, Empty, CwEnv, CwOrchUpload};
/// use cw_plus_orchestrate::Cw20Base;
/// use cw20::Cw20Coin;
///
/// pub struct MyApplication<Chain: CwEnv> {
///   pub token: Cw20Base<Chain>
/// }
///
/// impl<Chain: CwEnv> Deploy<Chain> for MyApplication<Chain> {
///     type Error = CwOrchError;
///     type DeployData = Empty;
///     fn store_on(chain: Chain) -> Result<Self, CwOrchError> {
///         let mut token = Cw20Base::new("my-token", chain.clone());
///         token.upload()?;
///         Ok(Self { token })
///     }
///     // deploys the token to the chain
///     fn deploy_on(chain: Chain, data: Empty) -> Result<Self, CwOrchError> {
///         let my_app: MyApplication<Chain> = Self::store_on(chain)?;
///         let cw20_init_msg = cw20_base::msg::InstantiateMsg {
///             decimals: 6,
///             name: "Test Token".to_string(),
///             initial_balances: vec![],
///             marketing: None,
///             mint: None,
///             symbol: "TEST".to_string(),
///         };
///         // instantiates the token and stores its address to the "my-token" key
///         my_app.token.instantiate(&cw20_init_msg, None, None)?;
///         Ok(my_app)
///    }
///    // loads the token from the chain
///    fn load_from(chain: Chain) -> Result<Self, CwOrchError> {
///        // loads the token and uses the "my-token" key to get its information
///         let token = Cw20Base::new("my-token", chain.clone());
///         Ok(Self { token })
///    }
/// }
/// ```
///
/// This allows other developers to re-use the application's deployment logic in their own tests.
/// Allowing them to build on the application's functionality without having to re-implement its deployment.
pub trait Deploy<Chain: CwEnv>: Sized {
    /// Error type returned by the deploy functions.  
    type Error: Error + From<CwEnvError>;
    /// Data required to deploy the application.
    type DeployData: Clone;
    /// Stores/uploads the application to the chain.
    fn store_on(chain: Chain) -> Result<Self, Self::Error>;
    /// Deploy the application to the chain. This could include instantiating contracts.
    #[allow(unused_variables)]
    fn deploy_on(chain: Chain, data: Self::DeployData) -> Result<Self, Self::Error> {
        // if not implemented, just store the application on the chain
        Self::store_on(chain)
    }

    /// Deploys the applications on all chains indicated in `chains`.
    /// Arguments :
    ///  - `networks`` is a vector of :
    ///     - Chain objects
    ///     - Additional deploy data needed for the deployment of the structure on each platform
    fn multi_network_deploy(
        networks: Vec<(Chain, Self::DeployData)>,
        _gas_needed: Option<u64>,
        after_deploy_action: Option<fn(&Self) -> anyhow::Result<()>>,
    ) -> anyhow::Result<HashMap<String, Self>> {
        let hash_networks: HashMap<String, (Chain, Self::DeployData)> = networks
            .iter()
            .map(|(c, d)| Ok::<_, Chain::Error>((c.block_info()?.chain_id, (c.clone(), d.clone()))))
            .collect::<Result<HashMap<_, _>, _>>()
            .map_err(Into::into)?;

        // First we check the deployment status. Which chains have been un-succesfully deployed since last time
        let chains_to_deploy = if let Ok(deployment_left) = read_deployment() {
            // We check the validity of the deployment_left variable against all our networks
            for chain in deployment_left.iter() {
                // If the deployment file contains a chain which is not in the networks variable, the deployment file is considered corrupted
                if !hash_networks.contains_key(chain) {
                    bail!("Deployment file is corrupted. Chain {} is indicated but not in the `networks` argument of the `ful_deploy`function", chain)
                }
            }

            hash_networks
                .iter()
                .filter(|(chain, _)| deployment_left.contains(chain))
                .map(|(chain_id, (chain, data))| (chain_id.clone(), (chain.clone(), data.clone())))
                .collect()
        } else {
            // There is not deployment file, we make sure the user wants to deploy to multiple chains
            if !CwOrchEnvVars::load()?.disable_manual_interaction {
                println!(
                    "Do you want to deploy to {:?}? Use 'n' to abort, 'y' to continue ",
                    &hash_networks.keys().cloned().collect::<Vec<String>>()
                );
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if input.to_lowercase().contains('n') {
                    bail!("Deployment aborted manually");
                }
            } else {
                log::info!(
                    "Deploying to all the following networks: {:?}",
                    &hash_networks.keys().cloned().collect::<Vec<String>>()
                );
            }

            hash_networks
        };

        // We update the deployment file with all the chains we want to use now
        let mut chains_left: HashSet<_> = chains_to_deploy.keys().cloned().collect();
        write_deployment(&chains_left)?;

        let mut deployments = HashMap::new();

        for (chain_id, (chain, data)) in chains_to_deploy {
            // First we check that there is enough funds to deploy the whole application + after_deploy_action
            // TODO

            let err = match Self::deploy_on(chain, data) {
                Ok(this_deployment) => {
                    // We execute the after deployment action if it exists
                    let after_deploy_action_result =
                        after_deploy_action.map(|action| action(&this_deployment));

                    match after_deploy_action_result {
                        None | Some(Ok(_)) => {
                            // We remove the chain from the deployment file and continue with the next iteration
                            chains_left.remove(&chain_id);
                            write_deployment(&chains_left)?;
                            deployments.insert(chain_id, this_deployment);
                            continue;
                        }
                        Some(Err(e)) => format!("Error in after deployment closure : {e}"),
                    }
                }
                Err(e) => e.to_string(),
            };
            log::error!("Deployment failed for chain {chain_id}, You can retry deployment running the `full_deploy` function again. Error log : {err}");
        }

        // If all deployments have gone through, we delete the deployments file
        if chains_left.is_empty() {
            remove_deployment_file()?;
        }
        Ok(deployments)
    }

    /// Set the default contract state for a contract, so that users can retrieve it in their application when importing the library
    /// If a state is provided, it is used for all contracts, otherwise, the state is loaded from the crate's state file.
    fn set_contracts_state(&mut self, custom_state: Option<Value>) {
        let state;

        let state_file = Self::deployed_state_file_path();
        if let Some(custom_state) = custom_state {
            state = custom_state;
        } else if let Some(state_file) = state_file {
            if let Ok(module_state_json) = read_json(&state_file) {
                state = module_state_json;
            } else {
                return;
            }
        } else {
            return;
        }

        let all_contracts = self.get_contracts_mut();

        for contract in all_contracts {
            // We set the code_id and/or address of the contract in question if they are not present already
            let deploy_details = contract.get_chain().state().deploy_details();
            // We load the file
            // We try to get the code_id for the contract
            if contract.code_id().is_err() {
                let code_id = state
                    .get(deploy_details.chain_name.clone())
                    .unwrap_or(&Value::Null)
                    .get(deploy_details.chain_id.to_string())
                    .unwrap_or(&Value::Null)
                    .get("code_ids")
                    .unwrap_or(&Value::Null)
                    .get(contract.id());

                if let Some(code_id) = code_id {
                    if code_id.is_u64() {
                        contract.set_default_code_id(code_id.as_u64().unwrap())
                    }
                }
            }
            // We try to get the address for the contract
            if contract.address().is_err() {
                // Try and get the code id from file
                let address = state
                    .get(deploy_details.chain_name.clone())
                    .unwrap_or(&Value::Null)
                    .get(deploy_details.chain_id.to_string())
                    .unwrap_or(&Value::Null)
                    .get(deploy_details.deployment_id)
                    .unwrap_or(&Value::Null)
                    .get(contract.id());

                if let Some(address) = address {
                    if address.is_string() {
                        contract.set_default_address(&Addr::unchecked(address.as_str().unwrap()))
                    }
                }
            }
        }
    }

    /// Gets all the chain ids on which the library is deployed on
    /// This loads all chains that are registered in the crate-local daemon_state file
    /// The state file should have the following format :
    /// {
    ///     "juno":{
    ///         "juno-1":{
    ///             ...
    ///         },
    ///         "uni-6": {

    ///         }
    ///     }
    ///     ...
    /// }
    /// So this function actually looks for the second level of indices in the deployed_state_file
    fn get_all_deployed_chains() -> Vec<String> {
        let deployed_state_file = Self::deployed_state_file_path();
        if let Some(state_file) = deployed_state_file {
            if let Ok(module_state_json) = read_json(&state_file) {
                let all_chain_ids: Vec<String> = module_state_json
                    .as_object()
                    .unwrap()
                    .into_iter()
                    .flat_map(|(_, v)| {
                        v.as_object()
                            .unwrap()
                            .into_iter()
                            .map(|(chain_id, _)| chain_id.clone())
                            .collect::<Vec<_>>()
                    })
                    .collect();

                return all_chain_ids;
            }
        }
        vec![]
    }

    /// Sets the custom state file path for exporting the state with the package.
    /// This function needs to be defined by projects. If the project doesn't want to give deployment state with their crate, they can return None here.
    fn deployed_state_file_path() -> Option<String>;

    /// Returns all the contracts in this deployment instance
    /// Used to set the contract state (addr and code_id) when importing the package.
    fn get_contracts_mut(&mut self) -> Vec<Box<&mut dyn ContractInstance<Chain>>>;
    /// Load the application from the chain, assuming it has already been deployed.
    /// In order to leverage the deployed state, don't forget to call `Self::set_contracts_state` after loading the contract objects
    fn load_from(chain: Chain) -> Result<Self, Self::Error>;
}

/// Read a json value from a file (redundant with crate::daemon::json_file, but returns an err instead of panicking)
pub(crate) fn read_json(filename: &String) -> anyhow::Result<Value> {
    let file = File::open(filename)?;
    let json: serde_json::Value = from_reader(file)?;
    Ok(json)
}

fn deployment_file() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".cw-orchestrator")
        .join("deployment.json")
}

fn write_deployment(status: &HashSet<String>) -> anyhow::Result<()> {
    let path = deployment_file();
    let vector: Vec<String> = status.iter().cloned().collect();
    let status_str = serde_json::to_string_pretty(&vector)?;
    fs::write(path, status_str)?;
    Ok(())
}

fn read_deployment() -> anyhow::Result<Vec<String>> {
    let path = deployment_file();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as a vector of chain ids. If not present use default.
    Ok(serde_json::from_reader(reader)?)
}

fn remove_deployment_file() -> anyhow::Result<()> {
    let path = deployment_file();
    remove_file(path)?;
    Ok(())
}
