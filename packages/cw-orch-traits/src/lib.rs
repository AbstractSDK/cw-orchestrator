use cw_orch_core::environment::CwEnv;

pub mod environment;
pub mod stargate;

pub use stargate::Stargate;
pub trait FullNode: CwEnv + Stargate {}

impl<C: CwEnv + Stargate> FullNode for C {}
