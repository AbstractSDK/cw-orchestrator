//! Analysis of committed IBC related Txs and Packets.

use crate::packet::success::{SuccessNestedPacketsFlow, SuccessSinglePacketFlow};
use crate::packet::{
    success::SuccessIbcPacketOutcome, IbcPacketOutcome, NestedPacketsFlow, SinglePacketFlow,
};
use crate::tx::TxId;
use crate::{IbcAckParser, InterchainError};
use cw_orch_core::environment::CwEnv;

/// Trait used for analysis of IBC packet flows
pub trait PacketAnalysis {
    /// Result of the Analysis of the packet flows
    type AnalysisResult;

    /// Asserts that there is no timeout packet inside the result structure.
    fn assert_no_timeout(&self) -> Result<(), InterchainError>;

    /// Tries to parses all acknowledgements into polytone, ics20 and ics004 acks.
    /// Errors if some packet doesn't conform to those results.
    fn into_result(self) -> Result<Self::AnalysisResult, InterchainError>;
}

impl<Chain: CwEnv> PacketAnalysis for TxId<Chain> {
    type AnalysisResult = Self;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        Ok(())
    }

    fn into_result(self) -> Result<Self::AnalysisResult, InterchainError> {
        Ok(self)
    }
}

impl<T: PacketAnalysis> PacketAnalysis for IbcPacketOutcome<T> {
    type AnalysisResult = SuccessIbcPacketOutcome<T::AnalysisResult>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        match &self {
            IbcPacketOutcome::Success { .. } => Ok(()),
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }

    fn into_result(self) -> Result<Self::AnalysisResult, InterchainError> {
        match self {
            IbcPacketOutcome::Success {
                receive_tx,
                ack_tx,
                ack,
            } => {
                let mut is_successful_ack = false;
                if IbcAckParser::polytone_ack(&ack).is_ok() {
                    is_successful_ack = true;
                }
                if IbcAckParser::ics20_ack(&ack).is_ok() {
                    is_successful_ack = true;
                }
                if IbcAckParser::ics004_ack(&ack).is_ok() {
                    is_successful_ack = true;
                }
                if is_successful_ack {
                    Ok(SuccessIbcPacketOutcome {
                        receive_tx: receive_tx.into_result()?,
                        ack_tx: ack_tx.into_result()?,
                        ack,
                    })
                } else {
                    Err(InterchainError::AckDecodingFailed(
                        ack.clone(),
                        String::from_utf8_lossy(ack.as_slice()).to_string(),
                    ))
                }
            }
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }
}
impl<Chain: CwEnv> PacketAnalysis for SinglePacketFlow<Chain> {
    type AnalysisResult = SuccessSinglePacketFlow<Chain>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        self.outcome.assert_no_timeout()
    }

    fn into_result(self) -> Result<Self::AnalysisResult, InterchainError> {
        let success = self.outcome.into_result()?;

        Ok(SuccessSinglePacketFlow {
            send_tx: self.send_tx,
            outcome: success,
        })
    }
}

impl<Chain: CwEnv> PacketAnalysis for NestedPacketsFlow<Chain> {
    type AnalysisResult = SuccessNestedPacketsFlow<Chain>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        self.packets
            .iter()
            .map(|p| p.assert_no_timeout())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn into_result(self) -> Result<Self::AnalysisResult, InterchainError> {
        let packets = self
            .packets
            .into_iter()
            .map(|p| p.into_result())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SuccessNestedPacketsFlow {
            tx_id: self.tx_id,
            packets,
        })
    }
}

impl<Chain: CwEnv> NestedPacketsFlow<Chain> {
    /// Returns all successful packets gathered during the packet following procedure
    /// Doesn't error if a packet has timed-out
    pub fn get_success_packets(self) -> Vec<SuccessSinglePacketFlow<Chain>> {
        self.packets
            .into_iter()
            .flat_map(|outcome| match outcome {
                IbcPacketOutcome::Success {
                    ack,
                    receive_tx,
                    ack_tx,
                } => [
                    vec![SuccessSinglePacketFlow {
                        send_tx: Some(self.tx_id.clone()),
                        outcome: SuccessIbcPacketOutcome {
                            receive_tx: receive_tx.tx_id.clone(),
                            ack_tx: ack_tx.tx_id.clone(),
                            ack,
                        },
                    }],
                    receive_tx.get_success_packets(),
                    ack_tx.get_success_packets(),
                ]
                .concat(),
                IbcPacketOutcome::Timeout { .. } => vec![],
            })
            .collect()
    }
}
