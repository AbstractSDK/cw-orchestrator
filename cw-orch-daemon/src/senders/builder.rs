use std::sync::Arc;

use cw_orch_core::environment::ChainInfoOwned;

use crate::DaemonError;

/// Allows building a `Sender` from `SenderBuilder::Options`
/// `async`` because it could do network requests during build
pub trait SenderBuilder {
    type Error: Into<DaemonError> + std::error::Error + std::fmt::Debug + Send + Sync + 'static;
    type Sender;

    /// Build a new `Sender`.
    fn build(
        &self,
        chain_info: &Arc<ChainInfoOwned>,
    ) -> impl std::future::Future<Output = Result<Self::Sender, Self::Error>> + Send;
}
