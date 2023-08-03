//! Integration testing execution environment backed by a [cw-multi-test](cw_multi_test) App.
//! It has an associated state that stores deployment information for easy retrieval and contract interactions.

mod core;
mod state;
mod custom;

pub use self::core::*;
pub use state::*;
pub use custom::*;
