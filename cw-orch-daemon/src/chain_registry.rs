use cw_orch_networks::networks::parse_network_safe;
use ibc_chain_registry::{chain::ChainData, fetchable::Fetchable};
use tokio::runtime::Handle;

pub fn parse_network_all(rt: &Handle, net_id: &str) -> Result<ChainData, String> {
    let local_network = parse_network_safe(net_id);
    if let Ok(network) = local_network {
        Ok(network.into())
    } else {
        // We look in the chain registry
        let chain_name = chain_name_from_chain_id(net_id)?;
        let registry_network = rt
            .block_on(ChainData::fetch(chain_name, None))
            .map_err(|_| local_network.unwrap_err())?;

        Ok(registry_network)
    }
}

fn chain_name_from_chain_id(net_id: &str) -> Result<String, String> {
    let parts: Vec<&str> = net_id.rsplitn(2, '-').collect();
    // the parts vector should look like [53159, cosmos-tesnet], because we are using rsplitn
    parts
        .get(1)
        .ok_or(format!("Wrong chain id format: {}", net_id))
        .map(|s| s.to_string())
}
