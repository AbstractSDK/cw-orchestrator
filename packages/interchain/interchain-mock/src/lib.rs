//! Implementation of the interchain traits for the [cw_orch::prelude::Mock] environment

mod error;
mod interchain;

use cosmwasm_std::testing::MockApi;
use cw_orch_mock::cw_multi_test::MockApiBech32;
pub use error::InterchainMockError;

pub type MockInterchainEnv = interchain::MockInterchainEnvBase<MockApi>;
pub type MockBech32InterchainEnv = interchain::MockInterchainEnvBase<MockApiBech32>;
