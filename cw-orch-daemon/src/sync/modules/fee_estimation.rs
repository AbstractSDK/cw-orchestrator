use cosmwasm_std::coin;
use cw_orch_core::environment::{modules::fee_estimation::FeeEstimation, ChainState};

use crate::Daemon;


impl FeeEstimation for Daemon{
    fn estimate_fee(&self, gas: u64) -> Result<cosmwasm_std::Coin, <Self as cw_orch_core::environment::TxHandler>::Error> {

        let chain_data = self.state().as_ref().chain_data.clone();

        let fee_token = chain_data.fees.fee_tokens[0].clone();
        let fee = (gas as f64 * fee_token.fixed_min_gas_price) as u128;
        
        Ok(coin(fee, fee_token.denom))
    }
}