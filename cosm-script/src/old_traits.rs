use crate::{contract::ContractInstance, error::CosmScriptError, CosmTxResponse};
use async_trait::async_trait;
use cosmrs::Coin;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct NotImplemented(String);

impl Default for NotImplemented {
    // TODO: improve, when rust allows negating trait bounds or overwriting traits
    fn default() -> Self {
        Self("Stupid Workaround".into())
    }
}

// Fn for custom implementation to return ContractInstance
pub trait Instance {
    fn instance(&self) -> &ContractInstance;
}

/// Implementing Interface ensures type safety
pub trait Interface {
    type Init: Serialize;
    type Exec: Serialize;
    type Query: Serialize;
    type Migrate: Serialize;
}

/// Smart Contract execute endpoint
#[async_trait(?Send)]
pub trait WasmExecute {
    type E: Serialize;

    async fn exec<'a>(
        &self,
        execute_msg: &'a Self::E,
        coins: Option<&[Coin]>,
    ) -> Result<CosmTxResponse, CosmScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmExecute for T {
    type E = <T as Interface>::Exec;

    async fn exec<'a>(
        &self,
        execute_msg: &'a Self::E,
        coins: Option<&[Coin]>,
    ) -> Result<CosmTxResponse, CosmScriptError> {
        assert_implemented(&execute_msg)?;
        self.instance()
            .execute(&execute_msg, coins.unwrap_or(&[]))
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
    ) -> Result<CosmTxResponse, CosmScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmInstantiate for T {
    type I = <T as Interface>::Init;

    async fn init(
        &self,
        instantiate_msg: Self::I,
        admin: Option<String>,
        coins: Option<&[Coin]>,
    ) -> Result<CosmTxResponse, CosmScriptError> {
        assert_implemented(&instantiate_msg)?;
        self.instance()
            .instantiate(instantiate_msg, admin, coins.unwrap_or_default())
            .await
    }
}

/// Smart Contract query endpoint

#[async_trait(?Send)]
pub trait WasmQuery {
    type Q: Serialize;

    async fn query<G: Serialize + DeserializeOwned>(
        &self,
        query_msg: Self::Q,
    ) -> Result<G, CosmScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmQuery for T {
    type Q = <T as Interface>::Query;

    async fn query<G: Serialize + DeserializeOwned>(
        &self,
        query_msg: Self::Q,
    ) -> Result<G, CosmScriptError> {
        assert_implemented(&query_msg)?;
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
    ) -> Result<CosmTxResponse, CosmScriptError>;
}

#[async_trait(?Send)]
impl<T: Interface + Instance> WasmMigrate for T {
    type M = <T as Interface>::Migrate;

    async fn migrate(
        &self,
        migrate_msg: Self::M,
        new_code_id: u64,
    ) -> Result<CosmTxResponse, CosmScriptError> {
        assert_implemented(&migrate_msg)?;
        self.instance().migrate(migrate_msg, new_code_id).await
    }
}

/// Smart Contract migrate endpoint

#[async_trait(?Send)]
pub trait WasmUpload {
    async fn upload(&self, path: &str) -> Result<CosmTxResponse, CosmScriptError>;
}

#[async_trait(?Send)]
impl<T: Instance> WasmUpload for T {
    async fn upload(&self, path: &str) -> Result<CosmTxResponse, CosmScriptError> {
        self.instance().upload(path).await
    }
}

// asserts that trait function is implemented for contract
fn assert_implemented<E: Serialize>(msg: &E) -> Result<(), CosmScriptError> {
    if serde_json::to_string(msg)? == serde_json::to_string(&NotImplemented::default())? {
        return Err(CosmScriptError::NotImplemented);
    }
    Ok(())
}

// TODO: find out how to create a wrapper trait that can be imported to expose all the interfaces
// pub trait WasmContract<'a>: WasmExecute + WasmInstantiate + WasmQuery + WasmMigrate {}

// /// Smart Contract execute endpoint
// #[async_trait(?Send)]
// pub trait WasmContract<'a>: WasmExecute + WasmInstantiate + WasmQuery where &'a Self: 'a + Interface + Instance  {
//     async fn exec<'b>(
//         &'a self,
//         execute_msg:&'b <&'a Self as Interface>::E,
//         coins: Option<&[Coin]>,
//     ) -> Result<CosmTxResponse, CosmScriptError>;
// }
