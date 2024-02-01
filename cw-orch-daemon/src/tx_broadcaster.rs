use std::time::Duration;

use bitcoin::secp256k1::All;
use cosmrs::proto::cosmos::base::abci::v1beta1::TxResponse;
use cw_orch_core::log::transaction_target;

use crate::{queriers::DaemonNodeQuerier, sender::Sender, CosmTxResponse, DaemonError, TxBuilder};

pub type StrategyAction =
    fn(&mut TxBuilder, &Result<TxResponse, DaemonError>) -> Result<(), DaemonError>;

pub struct RetryStrategy {
    /// This function is called right after a transaction has been submitted to a node.
    /// It is used to check if the transaction has been submitted successfully or if an error occurred
    /// This function returns true if an error corresponding to the current strategy is detected
    pub broadcast_condition: fn(&TxResponse) -> bool,
    /// This function is called when a simulation error occurs
    /// This function return true if an error corresponding to the current strategy is detected
    pub simulation_condition: fn(&DaemonError) -> bool,
    /// Once the algorithm detects an error when broadcasting/simulating a transaction, it triggers this action before re-trying tx submission
    /// This action takes the transaction builder and the transaction response result as arguments
    /// It should make changes to the tx builder object (which is passed as a mutable reference)
    /// The tx_builder object will be used after that to re-try submitting the transaction
    pub action: Option<StrategyAction>,
    pub max_retries: BroadcastRetry,
    pub(crate) current_retries: u64,
    pub reason: String,
}

impl RetryStrategy {
    pub fn new(
        broadcast_condition: fn(&TxResponse) -> bool,
        simulation_condition: fn(&DaemonError) -> bool,
        action: Option<StrategyAction>,
        max_retries: BroadcastRetry,
        reason: String,
    ) -> Self {
        RetryStrategy {
            broadcast_condition,
            simulation_condition,
            action,
            max_retries,
            current_retries: 0,
            reason,
        }
    }
}

#[derive(Default)]
pub struct TxBroadcaster {
    strategies: Vec<RetryStrategy>,
}

pub enum BroadcastRetry {
    Infinite,
    Finite(u64),
}

impl TxBroadcaster {
    /// Adds a retry strategy to the broadcaster
    /// Order of strategy addition matters, strategy conditions are tested in order of addition.
    /// Each time a transaction is retried, only the first retry strategy met is applied
    pub fn add_strategy(mut self, s: RetryStrategy) -> Self {
        self.strategies.push(s);
        self
    }

    // We can't make async recursions easily because wallet is not `Sync`
    // Thus we use a `while` loop structure here
    pub async fn broadcast(
        mut self,
        mut tx_builder: TxBuilder,
        wallet: &Sender<All>,
    ) -> Result<TxResponse, DaemonError> {
        let mut tx_retry = true;

        // We try and broadcast once
        let mut tx_response = broadcast_helper(&mut tx_builder, wallet).await;
        log::info!(
            target: &transaction_target(),
            "Awaiting TX inclusion in block..."
        );
        while tx_retry {
            tx_retry = false;

            // On error, we verify it matches the retry strategies. If it does, we update the retry counts and start over
            for s in self.strategies.iter_mut() {
                if strategy_condition_met(s, &tx_response) && can_retry(s) {
                    // We update the tx and resubmit
                    if let Some(action) = s.action {
                        action(&mut tx_builder, &tx_response)?;
                    }
                    tx_retry = true;

                    // We still await for the next block, to avoid spamming retry when an error occurs
                    let block_speed = DaemonNodeQuerier::new_async(wallet.channel())
                        ._average_block_speed(None)
                        .await?;
                    log::warn!(
                        target: &transaction_target(),
                        "Retrying broadcasting TX in {} seconds because of {}",
                        block_speed,
                        s.reason
                    );
                    tokio::time::sleep(Duration::from_secs(block_speed)).await;

                    tx_response = broadcast_helper(&mut tx_builder, wallet).await;
                    continue;
                }
            }
        }
        tx_response
    }
}

fn strategy_condition_met(
    s: &RetryStrategy,
    tx_response: &Result<TxResponse, DaemonError>,
) -> bool {
    match tx_response {
        Ok(r) => (s.broadcast_condition)(r),
        Err(e) => (s.simulation_condition)(e),
    }
}

async fn broadcast_helper(
    tx_builder: &mut TxBuilder,
    wallet: &Sender<All>,
) -> Result<TxResponse, DaemonError> {
    let tx = tx_builder.build(wallet).await?;
    let tx_response = wallet.broadcast_tx(tx).await?;
    log::debug!(target: &transaction_target(), "TX broadcast response: {:?}", tx_response);

    assert_broadcast_code_response(tx_response)
}

/// Tx Responses with a non 0 code, should also error with the raw loq
pub(crate) fn assert_broadcast_code_response(
    tx_response: TxResponse,
) -> Result<TxResponse, DaemonError> {
    // if tx result != 0 then the tx failed, so we return an error
    // if tx result == 0 then the tx succeeded, so we return the tx response
    if tx_response.code == 0 {
        Ok(tx_response)
    } else {
        Err(DaemonError::TxFailed {
            code: tx_response.code as usize,
            reason: tx_response.raw_log,
        })
    }
}

/// Tx Responses with a non 0 code, should also error with the raw loq
pub(crate) fn assert_broadcast_code_cosm_response(
    tx_response: CosmTxResponse,
) -> Result<CosmTxResponse, DaemonError> {
    // if tx result != 0 then the tx failed, so we return an error
    // if tx result == 0 then the tx succeeded, so we return the tx response
    if tx_response.code == 0 {
        Ok(tx_response)
    } else {
        Err(DaemonError::TxFailed {
            code: tx_response.code,
            reason: tx_response.raw_log,
        })
    }
}

fn can_retry(s: &mut RetryStrategy) -> bool {
    match s.max_retries {
        BroadcastRetry::Infinite => true,
        BroadcastRetry::Finite(max_retries) => {
            s.current_retries += 1;
            s.current_retries <= max_retries
        }
    }
}

fn has_insufficient_fee(raw_log: &str) -> bool {
    raw_log.contains("insufficient fees")
}

// from logs: "insufficient fees; got: 14867ujuno
// required: 17771ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9,444255ujuno: insufficient fee"
fn parse_suggested_fee(raw_log: &str) -> Option<u128> {
    // Step 1: Split the log message into "got" and "required" parts.
    let parts: Vec<&str> = raw_log.split("required: ").collect();

    // Make sure the log message is in the expected format.
    if parts.len() != 2 {
        return None;
    }

    // Step 2: Split the "got" part to extract the paid fee and denomination.
    let got_parts: Vec<&str> = parts[0].split_whitespace().collect();

    // Extract the paid fee and denomination.
    let paid_fee_with_denom = got_parts.last()?;
    let (_, denomination) =
        paid_fee_with_denom.split_at(paid_fee_with_denom.find(|c: char| !c.is_numeric())?);

    eprintln!("denom: {}", denomination);

    // Step 3: Iterate over each fee in the "required" part.
    let required_fees: Vec<&str> = parts[1].split(denomination).collect();

    eprintln!("required fees: {:?}", required_fees);

    // read until the first non-numeric character backwards on the first string
    let (_, suggested_fee) =
        required_fees[0].split_at(required_fees[0].rfind(|c: char| !c.is_numeric())?);
    eprintln!("suggested fee: {}", suggested_fee);

    // remove the first character if parsing errors, which can be a comma
    suggested_fee
        .parse::<u128>()
        .ok()
        .or(suggested_fee[1..].parse::<u128>().ok())
}

pub fn insufficient_fee_strategy() -> RetryStrategy {
    RetryStrategy::new(
        |tx_response| has_insufficient_fee(&tx_response.raw_log),
        |_| false, // Simulation doesn't have gas issues
        Some(|tx_builder, tx_response| {
            // get the suggested fee from the error message
            // If we enter this function for this specific strategy, it's because something was detected in the tx response object
            let suggested_fee = parse_suggested_fee(&tx_response.as_ref().unwrap().raw_log);

            let Some(new_fee) = suggested_fee else {
                return Err(DaemonError::InsufficientFee(
                    tx_response.as_ref().unwrap().raw_log.clone(),
                ));
            };

            // update the fee and try again
            tx_builder.fee_amount(new_fee);

            Ok(())
        }),
        BroadcastRetry::Finite(1),
        "an insufficient fee error".to_string(),
    )
}

fn has_account_sequence_error(raw_log: &str) -> bool {
    raw_log.contains("incorrect account sequence")
}

pub fn account_sequence_strategy() -> RetryStrategy {
    RetryStrategy::new(
        |tx_response| has_account_sequence_error(&tx_response.raw_log),
        |simulation_error| has_account_sequence_error(&simulation_error.to_string()),
        None,
        BroadcastRetry::Infinite,
        "an account sequence error".to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_suggested_fee() {
        let log = "insufficient fees; got: 14867ujuno required: 17771ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9,444255ujuno: insufficient fee";
        let fee = parse_suggested_fee(log).unwrap();
        assert_eq!(fee, 444255);
    }
}
