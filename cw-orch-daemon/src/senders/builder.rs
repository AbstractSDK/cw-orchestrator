use cw_orch_core::environment::ChainInfoOwned;

use crate::DaemonError;

/// Allows building a `Sender` from `SenderBuilder::Options`
/// `async`` because it could do network requests during build
pub trait SenderBuilder: Clone {
    type Error: Into<DaemonError> + std::error::Error + std::fmt::Debug + Send + Sync + 'static;
    /// Options for the sender
    type Options: Default + Clone;

    /// Build a new `Sender`.
    fn build(
        chain_info: ChainInfoOwned,
        sender_options: Self::Options,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send;
}
