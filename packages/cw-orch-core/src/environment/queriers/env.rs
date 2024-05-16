#[derive(Clone, Debug)]
pub struct EnvironmentInfo {
    pub chain_id: String,
    pub chain_name: String,
    pub deployment_id: String,
}

pub trait EnvironmentQuerier {
    /// Get some details about the environment.
    fn env_info(&self) -> EnvironmentInfo;
}
