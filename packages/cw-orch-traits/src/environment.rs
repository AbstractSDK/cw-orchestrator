use cw_orch_core::{contract::interface_traits::ContractInstance, environment::CwEnv};

pub trait Environment<Chain: CwEnv> {
    // Required method
    fn environment(&self) -> Chain;
}

impl<Chain: CwEnv, T: ContractInstance<Chain>> Environment<Chain> for T {
    fn environment(&self) -> Chain {
        self.get_chain().clone()
    }
}
