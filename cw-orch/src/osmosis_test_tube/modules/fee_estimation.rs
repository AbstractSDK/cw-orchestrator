use cosmwasm_std::coin;
use cw_orch_core::environment::modules::fee_estimation::FeeEstimation;
use osmosis_test_tube::FeeSetting;

use crate::prelude::OsmosisTestTube;

impl FeeEstimation for OsmosisTestTube{
    fn estimate_fee(&self, gas: u64) -> Result<cosmwasm_std::Coin, <Self as cw_orch_core::environment::TxHandler>::Error> {
        let fee = self.sender.fee_setting();

        let fee = match fee {
            FeeSetting::Auto {
                gas_price,
                gas_adjustment,
            } => coin(
                (((gas as f64) * gas_adjustment) as u128) * gas_price.amount.u128(),
                gas_price.denom.clone(),
            ),
            FeeSetting::Custom { amount, .. } => amount.clone(),
        };
        Ok(fee)
    }
}