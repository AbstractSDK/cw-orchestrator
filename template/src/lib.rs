use boot_core::{Contract, IndexResponse, TxHandler};
use serde::Serialize;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

// Update `MyProjectName` to your project name and export contract implementations here.
// No need to touch anything else

pub struct MyProjectName<
    Chain: TxHandler,
    E: Serialize + Debug,
    I: Serialize + Debug,
    Q: Serialize + Debug,
    M: Serialize + Debug,
>(Contract<Chain, E, I, Q, M>)
where
    <Chain as TxHandler>::Response: IndexResponse;

impl<
        Chain: TxHandler,
        E: Serialize + Debug,
        I: Serialize + Debug,
        Q: Serialize + Debug,
        M: Serialize + Debug,
    > Deref for MyProjectName<Chain, E, I, Q, M>
where
    <Chain as TxHandler>::Response: IndexResponse,
{
    type Target = Contract<Chain, E, I, Q, M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<
        Chain: TxHandler,
        E: Serialize + Debug,
        I: Serialize + Debug,
        Q: Serialize + Debug,
        M: Serialize + Debug,
    > DerefMut for MyProjectName<Chain, E, I, Q, M>
where
    <Chain as TxHandler>::Response: IndexResponse,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
