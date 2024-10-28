//! Integration testing execution environment backed by a [osmosis-test-tube](osmosis_test_tube) App.
//! It has an associated state that stores deployment information for easy retrieval and contract interactions.
mod core;

mod queriers;
pub use self::core::*;
