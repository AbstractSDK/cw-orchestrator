//! Easy to use CosmWasm-plus scripting library
//!
//! Provides an abstraction over a queue.  When the abstraction is used
//! there are these advantages:
//! - Fast
//! - [`Easy`]
//!
//! [`Easy`]: http://thatwaseasy.example.com

pub(crate) mod cw20;

use std::ops::{Deref, DerefMut};

pub use crate::cw20::Cw20;

use boot_derive::Boot;
use cosm_script::{contract::Contract, index_response::IndexResponse, tx_handler::TxHandler};
use serde::Serialize;
use std::fmt::Debug;

// Newtype
// unit-struct wouldn't compile properly

pub struct CwPlusContract<
    Chain: TxHandler,
    E: Serialize + Debug,
    I: Serialize + Debug,
    Q: Serialize,
    M: Serialize,
>(Contract<Chain, E, I, Q, M>)
where
    <Chain as TxHandler>::Response: IndexResponse;

// Generally considered bad practice but best solution rn.
// Circumventing the Orphan rule
impl<Chain: TxHandler, E: Serialize + Debug, I: Serialize + Debug, Q: Serialize, M: Serialize> Deref
    for CwPlusContract<Chain, E, I, Q, M>
where
    <Chain as TxHandler>::Response: IndexResponse,
{
    type Target = Contract<Chain, E, I, Q, M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Generally considered bad practice but best solution rn.
// Circumventing the Orphan rule
impl<Chain: TxHandler, E: Serialize + Debug, I: Serialize + Debug, Q: Serialize, M: Serialize>
    DerefMut for CwPlusContract<Chain, E, I, Q, M>
where
    <Chain as TxHandler>::Response: IndexResponse,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
