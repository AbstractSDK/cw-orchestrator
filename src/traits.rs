use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use terra_rust_api::{client::tx_types::V1TXResult, core_types::Coin};

use crate::{contract::ContractInstance, error::TerraRustScriptError};

// Fn for custom implementation to return ContractInstance
pub trait Instance {
    fn instance(&self) -> ContractInstance;
}

/// Implementing Interface ensures type safety
pub trait Interface {
    type I: Serialize;
    type E: Serialize;
    type Q: Serialize;
    type M: Serialize;
}

/// Smart Contract execute endpoint
#[async_trait(?Send)]
pub trait WasmExecute {
    type E: Serialize;

    async fn exec<'a>(
        &self,
        execute_msg: &'a Self::E,
        coins: Option<&Vec<Coin>>,
    ) -> Result<V1TXResult, TerraRustScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmExecute for T {
    type E = <T as Interface>::E;

    async fn exec<'a>(
        &self,
        execute_msg: &'a Self::E,
        coins: Option<&Vec<Coin>>,
    ) -> Result<V1TXResult, TerraRustScriptError> {
        self.instance()
            .execute(&execute_msg, coins.unwrap_or(&vec![]))
            .await
    }
}

/// Smart Contract instantiate endpoint

#[async_trait(?Send)]
pub trait WasmInstantiate {
    type I: Serialize;

    async fn init(
        &self,
        instantiate_msg: Self::I,
        admin: Option<String>,
        coins: Option<Vec<Coin>>,
    ) -> Result<V1TXResult, TerraRustScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmInstantiate for T {
    type I = <T as Interface>::I;

    async fn init(
        &self,
        instantiate_msg: Self::I,
        admin: Option<String>,
        coins: Option<Vec<Coin>>,
    ) -> Result<V1TXResult, TerraRustScriptError> {
        self.instance()
            .instantiate(instantiate_msg, admin, coins.unwrap_or_default())
            .await
    }
}

/// Smart Contract query endpoint

#[async_trait(?Send)]
pub trait WasmQuery {
    type Q: Serialize;

    async fn query(&self, query_msg: Self::Q) -> Result<Value, TerraRustScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmQuery for T {
    type Q = <T as Interface>::Q;

    async fn query(&self, query_msg: Self::Q) -> Result<Value, TerraRustScriptError> {
        self.instance().query(query_msg).await
    }
}

/// Smart Contract migrate endpoint

#[async_trait(?Send)]
pub trait WasmMigrate {
    type M: Serialize;

    async fn migrate(
        &self,
        migrate_msg: Self::M,
        new_code_id: u64,
    ) -> Result<V1TXResult, TerraRustScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmMigrate for T {
    type M = <T as Interface>::M;

    async fn migrate(
        &self,
        migrate_msg: Self::M,
        new_code_id: u64,
    ) -> Result<V1TXResult, TerraRustScriptError> {
        self.instance().migrate(migrate_msg, new_code_id).await
    }
}
