//! Easy to use CosmWasm-plus scripting library
//!
//! Provides an abstraction over a queue.  When the abstraction is used
//! there are these advantages:
//! - Fast
//! - [`Easy`]
//!
//! [`Easy`]: http://thatwaseasy.example.com

pub(crate) mod cw1;
pub(crate) mod cw20;

use std::ops::{Deref, DerefMut};

pub use crate::cw1::Cw1;
pub use crate::cw20::Cw20;
mod registry;
use boot_core::{Contract, IndexResponse, TxHandler};
pub use registry::*;
use serde::Serialize;
use std::fmt::Debug;

// Newtype
// unit-struct wouldn't compile properly
pub struct CwPlusContract<
    Chain: TxHandler,
    E: Serialize + Debug,
    I: Serialize + Debug,
    Q: Serialize + Debug,
    M: Serialize + Debug,
>(Contract<Chain, E, I, Q, M>)
where
    <Chain as TxHandler>::Response: IndexResponse;

// Generally considered bad practice but best solution rn.
// Circumventing the Orphan rule
impl<
        Chain: TxHandler,
        E: Serialize + Debug,
        I: Serialize + Debug,
        Q: Serialize + Debug,
        M: Serialize + Debug,
    > Deref for CwPlusContract<Chain, E, I, Q, M>
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
impl<
        Chain: TxHandler,
        E: Serialize + Debug,
        I: Serialize + Debug,
        Q: Serialize + Debug,
        M: Serialize + Debug,
    > DerefMut for CwPlusContract<Chain, E, I, Q, M>
where
    <Chain as TxHandler>::Response: IndexResponse,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
