//! Analysis of committed IBC related Txs and Packets.

use crate::packet::success::{SuccessNestedPacketsFlow, SuccessSinglePacketFlow};
use crate::packet::{
    success::SuccessIbcPacketOutcome, IbcPacketOutcome, NestedPacketsFlow, SinglePacketFlow,
};
use crate::tx::TxId;
use crate::{IbcAckParser, InterchainError};
use cosmwasm_std::{Binary, Empty};
use cw_orch_core::environment::CwEnv;

/// Trait used for analysis of IBC packet flows
pub trait PacketAnalysis {
    /// Result of the Analysis of the packet flows
    type AnalysisResult<CustomOutcome>;

    /// Asserts that there is no timeout packet inside the result structure.
    fn assert_no_timeout(&self) -> Result<(), InterchainError>;

    /// Tries to parses all acknowledgements into polytone, ics20 and ics004 acks.
    /// Errors if some packet doesn't conform to those results.
    fn into_result(self) -> Result<Self::AnalysisResult<Empty>, InterchainError>;

    /// Tries to parses all acknowledgements into polytone, ics20 and ics004 acks and additional provided parsing functions.
    /// Errors if some packet doesn't conform to those results.
    fn into_result_custom<CustomOutcome>(
        self,
        parse_func: fn(&Binary) -> Result<CustomOutcome, InterchainError>,
    ) -> Result<Self::AnalysisResult<CustomOutcome>, InterchainError>;
}

impl<Chain: CwEnv> PacketAnalysis for TxId<Chain, Empty> {
    type AnalysisResult<CustomOutcome> = TxId<Chain, CustomOutcome>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        Ok(())
    }

    fn into_result(self) -> Result<Self::AnalysisResult<Empty>, InterchainError> {
        Ok(self)
    }

    fn into_result_custom<CustomOutcome>(
        self,
        _funcs: fn(&Binary) -> Result<CustomOutcome, InterchainError>,
    ) -> Result<Self::AnalysisResult<CustomOutcome>, InterchainError> {
        Ok(TxId::new(self.chain_id, self.response))
    }
}

impl<T: PacketAnalysis> PacketAnalysis for IbcPacketOutcome<T> {
    type AnalysisResult<CustomOutcome> =
        SuccessIbcPacketOutcome<T::AnalysisResult<CustomOutcome>, CustomOutcome>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        match &self {
            IbcPacketOutcome::Success { .. } => Ok(()),
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }

    fn into_result(self) -> Result<Self::AnalysisResult<Empty>, InterchainError> {
        match self {
            IbcPacketOutcome::Success {
                receive_tx,
                ack_tx,
                ack,
            } => {
                let successful_ack = IbcAckParser::any_standard_ack(&ack)?;
                Ok(SuccessIbcPacketOutcome {
                    receive_tx: receive_tx.into_result()?,
                    ack_tx: ack_tx.into_result()?,
                    ack: successful_ack,
                })
            }
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }

    fn into_result_custom<CustomOutcome>(
        self,
        parsing_func: fn(&Binary) -> Result<CustomOutcome, InterchainError>,
    ) -> Result<Self::AnalysisResult<CustomOutcome>, InterchainError> {
        match self {
            IbcPacketOutcome::Success {
                receive_tx,
                ack_tx,
                ack,
            } => {
                let successful_ack =
                    IbcAckParser::any_standard_ack_with_custom(&ack, parsing_func)?;
                Ok(SuccessIbcPacketOutcome {
                    receive_tx: receive_tx.into_result_custom(parsing_func)?,
                    ack_tx: ack_tx.into_result_custom(parsing_func)?,
                    ack: successful_ack,
                })
            }
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }
}
impl<Chain: CwEnv> PacketAnalysis for SinglePacketFlow<Chain> {
    type AnalysisResult<CustomOutcome> = SuccessSinglePacketFlow<Chain, CustomOutcome>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        self.outcome.assert_no_timeout()
    }

    fn into_result(self) -> Result<Self::AnalysisResult<Empty>, InterchainError> {
        let success = self.outcome.into_result()?;

        Ok(SuccessSinglePacketFlow {
            send_tx: self.send_tx,
            outcome: success,
        })
    }

    fn into_result_custom<CustomOutcome>(
        self,
        parse_func: fn(&Binary) -> Result<CustomOutcome, InterchainError>,
    ) -> Result<Self::AnalysisResult<CustomOutcome>, InterchainError> {
        let success = self.outcome.into_result_custom(parse_func)?;

        Ok(SuccessSinglePacketFlow::<Chain, CustomOutcome> {
            send_tx: self.send_tx,
            outcome: success,
        })
    }
}

impl<Chain: CwEnv> PacketAnalysis for NestedPacketsFlow<Chain> {
    type AnalysisResult<CustomOutcome> = SuccessNestedPacketsFlow<Chain, CustomOutcome>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        self.packets
            .iter()
            .map(|p| p.assert_no_timeout())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn into_result(self) -> Result<Self::AnalysisResult<Empty>, InterchainError> {
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

    fn into_result_custom<CustomOutcome>(
        self,
        parse_func: fn(&Binary) -> Result<CustomOutcome, InterchainError>,
    ) -> Result<Self::AnalysisResult<CustomOutcome>, InterchainError> {
        let packets = self
            .packets
            .into_iter()
            .map(|p| p.into_result_custom(parse_func))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SuccessNestedPacketsFlow {
            tx_id: self.tx_id,
            packets,
        })
    }
}
