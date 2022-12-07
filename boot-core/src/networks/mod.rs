mod juno;
mod osmosis;
pub use super::daemon::state::{ChainInfo, NetworkInfo, NetworkKind};
pub use juno::{JUNO_1, LOCAL_JUNO, UNI_5};
pub use osmosis::{LOCAL_OSMO, OSMO_4};
