//! Integration testing execution environment backed by a [cw-remote-test](cw_remote_test) App.
//! It has an associated state that stores deployment information for easy retrieval and contract interactions.

mod core;

pub use self::core::RemoteMock;
