use cosmwasm_std::Addr;

use crate::CosmScriptError;

pub trait ChainState {
    type Out: StateInterface;
    fn state(&self) -> Self::Out;
}

pub trait StateInterface {
    fn get_address(&self, contract_id: &str) -> Result<Addr, CosmScriptError>;
    fn set_address(&mut self, contract_id: &str, address: &Addr);
    fn get_code_id(&self, contract_id: &str) -> Result<u64, CosmScriptError>;
    fn set_code_id(&mut self, contract_id: &str, code_id: u64);
}

// pub struct DaemonState{
//     pub path: String,
// }

// impl StateInterface for DaemonState {
//     fn address(&self, key: &str) -> String {
//         todo!()
//     }

//     fn save_address(&self,contract_id: &str, address: &str) {
//         todo!()
//     }
// }
