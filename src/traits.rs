use async_trait::async_trait;
use cosmrs::{Tx};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use crate::{contract::ContractInstance, error::TerraRustScriptError, core_types::Coin};

// Wrapper around the cli implementation for the specific network
#[async_trait(?Send)]

pub trait CliInterface {
    // cli command
    fn command(&self) -> &str;
    async fn query(&self);
    // async fn upload(&self);
    // async fn execute(&self);
    // async fn instantiate(&self);
}

// Fn for custom implementation to return ContractInstance
pub trait Instance {
    fn instance(&self) -> &ContractInstance;
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
        coins: Option<&[Coin]>,
    ) -> Result<Tx, TerraRustScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmExecute for T {
    type E = <T as Interface>::E;

    async fn exec<'a>(
        &self,
        execute_msg: &'a Self::E,
        coins: Option<&[Coin]>,
    ) -> Result<Tx, TerraRustScriptError> {
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
        coins: Option<&[Coin]>,
    ) -> Result<Tx, TerraRustScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmInstantiate for T {
    type I = <T as Interface>::I;

    async fn init(
        &self,
        instantiate_msg: Self::I,
        admin: Option<String>,
        coins: Option<&[Coin]>,
    ) -> Result<Tx, TerraRustScriptError> {
        self.instance()
            .instantiate(instantiate_msg, admin, coins.unwrap_or_default())
            .await
    }
}

/// Smart Contract query endpoint

#[async_trait(?Send)]
pub trait WasmQuery {
    type Q: Serialize;

    async fn query<T: Serialize + DeserializeOwned>(&self, query_msg: Self::Q) -> Result<T, TerraRustScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmQuery for T {
    type Q = <T as Interface>::Q;

    async fn query<R: Serialize + DeserializeOwned>(&self, query_msg: Self::Q) -> Result<R, TerraRustScriptError> {
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
    ) -> Result<Tx, TerraRustScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmMigrate for T {
    type M = <T as Interface>::M;

    async fn migrate(
        &self,
        migrate_msg: Self::M,
        new_code_id: u64,
    ) -> Result<Tx, TerraRustScriptError> {
        self.instance().migrate(migrate_msg, new_code_id).await
    }
}
