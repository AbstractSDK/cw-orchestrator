use cw_orch_core::environment::CwEnv;
use stargate::Stargate;

pub mod stargate;

pub trait FullNode: CwEnv + Stargate {}

impl<C: CwEnv + Stargate> FullNode for C {}
