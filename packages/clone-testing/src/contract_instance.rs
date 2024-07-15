use cw_orch_core::{
    contract::interface_traits::{ContractInstance, Uploadable},
    environment::{Environment, TxHandler},
    CwEnvError,
};

use crate::CloneTesting;

pub trait WasmUpload<Chain: TxHandler>: Uploadable + ContractInstance<Chain> {
    fn upload_wasm(&self) -> Result<<Chain as TxHandler>::Response, CwEnvError>;
}

impl<T> WasmUpload<CloneTesting> for T
where
    T: Uploadable + ContractInstance<CloneTesting>,
{
    fn upload_wasm(&self) -> Result<<CloneTesting as TxHandler>::Response, CwEnvError> {
        self.environment().upload_wasm(self)
    }
}
