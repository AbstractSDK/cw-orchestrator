//! Introduces the Deploy trait only
use crate::prelude::*;
use cosmwasm_std::Addr;
use serde_json::from_reader;
use serde_json::Value;
use std::fs::File;

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
    type Error: From<CwOrchError>;
    /// Data required to deploy the application.
    type DeployData;
    /// Stores/uploads the application to the chain.
    fn store_on(chain: Chain) -> Result<Self, Self::Error>;
    /// Deploy the application to the chain. This could include instantiating contracts.
    #[allow(unused_variables)]
    fn deploy_on(chain: Chain, data: Self::DeployData) -> Result<Self, Self::Error> {
        // if not implemented, just store the application on the chain
        Self::store_on(chain)
    }

    /// Set the default contract state for a contract, so that users can retrieve it in their application when importing the library
    fn set_contracts_state(&mut self) {
        let state_file = self.deployed_state_file_path();
        let all_contracts = self.get_contracts_mut();
        if let Some(state_file) = state_file {
            if let Ok(module_state_json) = read_json(&state_file) {
                for contract in all_contracts {
                    // We set the code_id and/or address of the contract in question if they are not present already
                    let deploy_details = contract.get_chain().state().deploy_details();
                    // We load the file
                    // We try to get the code_id for the contract
                    if contract.code_id().is_err() {
                        let code_id = module_state_json
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
                        let address = module_state_json
                            .get(deploy_details.chain_name.clone())
                            .unwrap_or(&Value::Null)
                            .get(deploy_details.chain_id.to_string())
                            .unwrap_or(&Value::Null)
                            .get(deploy_details.deployment_id)
                            .unwrap_or(&Value::Null)
                            .get(contract.id());

                        if let Some(address) = address {
                            if address.is_string() {
                                contract.set_default_address(&Addr::unchecked(
                                    address.as_str().unwrap(),
                                ))
                            }
                        }
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
    fn get_all_deployed_chains(&self) -> Vec<String> {
        let deployed_state_file = self.deployed_state_file_path();
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
    // TODO, we might want to enforce the projects to redefine this function ?
    fn deployed_state_file_path(&self) -> Option<String>;

    /// Returns all the contracts in this deployment instance
    /// Used to set the contract state (addr and code_id) when importing the package.
    fn get_contracts_mut(&mut self) -> Vec<Box<&mut dyn ContractInstance<Chain>>>;
    /// Load the application from the chain, assuming it has already been deployed.
    fn load_from(chain: Chain) -> Result<Self, Self::Error>;
}

/// Read a json value from a file (redundant with crate::daemon::json_file, but returns an err instead of panicking)
pub fn read_json(filename: &String) -> anyhow::Result<Value> {
    let file = File::open(filename)?;
    let json: serde_json::Value = from_reader(file)?;
    Ok(json)
}
