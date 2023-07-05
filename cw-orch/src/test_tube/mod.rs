//! Integration testing execution environment backed by a [test-tube](test_tube) App.
//! It has an associated state that stores deployment information for easy retrieval and contract interactions.

mod core;

pub use self::core::*;
