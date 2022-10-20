use std::ops::Deref;

use boot_core::{BootError, Contract, IndexResponse, TxHandler, TxResponse};
use cosmwasm_std::{Addr, Binary, Empty, Uint128};
use cw_multi_test::{ContractWrapper};
use cw1_whitelist::msg::*;
use crate::CwPlusContract;
use boot_core::Daemon;
pub type Cw1<Chain> = CwPlusContract<Chain, ExecuteMsg, InstantiateMsg, QueryMsg, Empty>;
// implement chain-generic functions
impl<Chain: TxHandler + Clone> Cw1<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(id: &str, chain: &Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw1_whitelist.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(ContractWrapper::new_with_empty(
                    cw20_base::contract::execute,
                    cw20_base::contract::instantiate,
                    cw20_base::contract::query,
                )))
                .with_wasm_path(file_path),
        )
    }
    pub fn set_path(self,path: &str) -> Self {
        Self(self.0.with_wasm_path(path))
    }
}