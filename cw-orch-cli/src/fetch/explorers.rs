use ibc_chain_registry::fetchable::Fetchable;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Explorers {
    pub explorers: Vec<Explorer>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Explorer {
    pub kind: String,
    pub url: String,
    pub tx_page: Option<String>,
    pub account_page: Option<String>,
}

impl Fetchable for Explorers {
    const DESC: &'static str = "Getting explorers list";

    fn path(resource: &str) -> PathBuf {
        [resource, "chain.json"].iter().collect()
    }
}
