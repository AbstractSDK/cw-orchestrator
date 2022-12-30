mod juno;
mod osmosis;
pub mod terra;
pub use super::daemon::state::{ChainInfo, NetworkInfo, NetworkKind};
pub use juno::{JUNO_1, LOCAL_JUNO, UNI_5};
pub use osmosis::{LOCAL_OSMO, OSMO_4};

// https://polkachu.com/testnet_public_grpc
// https://polkachu.com/public_grpc
