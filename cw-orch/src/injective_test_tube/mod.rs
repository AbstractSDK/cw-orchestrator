//! Integration testing execution environment backed by a [injective-test-tube](injective_test_tube) App.
//! It has an associated state that stores deployment information for easy retrieval and contract interactions.

mod core;

mod queriers;
pub use self::core::*;
