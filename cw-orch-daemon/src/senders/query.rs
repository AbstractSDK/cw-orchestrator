use crate::Channel;

use crate::DaemonError;

use super::builder::SenderBuilder;

/// A sender that can query information over a connection.
pub trait QuerySender: Clone {
    type Error: Into<DaemonError> + std::error::Error + std::fmt::Debug + Send + Sync + 'static;
    /// Options for this sender
    type Options: SenderBuilder<Sender = Self>;

    /// Get the channel for the sender
    fn channel(&self) -> Channel;
}
