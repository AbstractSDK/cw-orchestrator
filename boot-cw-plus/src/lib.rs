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
pub use crate::cw1::Cw1;
pub use crate::cw20::Cw20;
mod registry;
