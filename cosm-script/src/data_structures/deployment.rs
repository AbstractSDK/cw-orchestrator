use serde_json::{json, to_value, Value};

use crate::error::CosmScriptError;

use super::network::Network;

#[derive(Clone, Debug)]
pub struct Deployment {
    pub network: Network,
    pub name: String,
    pub proposal: bool,
}

impl Deployment {
    pub async fn new(name: String, network: Network, proposal: bool) -> anyhow::Result<Deployment> {
        check_deployment_existance(&name, &network)?;
        Ok(Deployment {
            network,
            name,
            proposal,
        })
    }

    pub fn get(&self) -> Result<Value, CosmScriptError> {
        Ok(self.network.get()?["deployments"][&self.name].clone())
    }

    pub fn set(&self, deployment: Value) -> Result<(), CosmScriptError> {
        let mut network = self.network.get()?;
        network["deployments"][&self.name] = deployment;
        self.network.set(network)
    }

    /// Get the contract address in the current deployment
    pub fn get_contract_address(&self, contract_name: &str) -> Result<String, CosmScriptError> {
        let deployment = self.get()?;
        let maybe_addr = deployment.get(contract_name);
        match maybe_addr {
            Some(addr) => Ok(addr.as_str().unwrap().into()),
            None => Err(CosmScriptError::AddrNotInFile(contract_name.to_owned())),
        }
    }

    /// Set the contract address in the current deployment
    pub fn save_contract_address(
        &self,
        contract_name: &str,
        contract_address: &str,
    ) -> Result<(), CosmScriptError> {
        let mut deployment = self.get()?;
        deployment[contract_name] = to_value(contract_address)?;
        self.set(deployment)
    }
}
#[inline]
fn check_deployment_existance(name: &str, network: &Network) -> anyhow::Result<()> {
    let mut cfg = network.get()?;
    let maybe_deployment = cfg["deployments"].get(name);
    match maybe_deployment {
        Some(_) => Ok(()),
        None => {
            cfg["deployments"][name] = json!({});
            network.set(cfg)?;
            Ok(())
        }
    }
}
