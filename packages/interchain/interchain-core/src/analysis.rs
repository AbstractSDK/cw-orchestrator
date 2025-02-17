//! Analysis of committed IBC related Txs and Packets.

use crate::packet::success::{SuccessNestedPacketsFlow, SuccessSinglePacketFlow};
use crate::packet::{
    success::IbcPacketResult, IbcPacketOutcome, NestedPacketsFlow, SinglePacketFlow,
};
use crate::tx::TxId;
use crate::{IbcAckParser, InterchainError};
use cosmwasm_std::{Binary, Empty};
use cw_orch_core::environment::CwEnv;
use cw_orch_core::environment::IndexResponse;

/// Trait used for analysis of IBC packet flows
pub trait PacketAnalysis: IndexResponse {
    /// Result of the Analysis of the packet flows
    type AnalysisResult<CustomResult>: IndexResponse;

    /// Asserts that there is no timeout packet inside the result structure.
    fn assert_no_timeout(&self) -> Result<(), InterchainError>;

    /// Tries to parses all acknowledgements into standard acknowledgments (polytone, ics20 or ics004).
    /// Errors if some packet doesn't conform to those results.
    fn assert(self) -> Result<Self::AnalysisResult<Empty>, InterchainError>;

    /// Tries to parses all acknowledgements into `CustomResult` using a custom parsing function.
    ///
    /// If it fails, also tries with standard acknowledgements (polytone, ics20 or ics004).
    /// Errors if some packet doesn't conform to those results.
    fn assert_custom<CustomResult>(
        self,
        parse_func: fn(&Binary) -> Result<CustomResult, InterchainError>,
    ) -> Result<Self::AnalysisResult<CustomResult>, InterchainError>;
}

impl<Chain: CwEnv> PacketAnalysis for TxId<Chain, Empty> {
    type AnalysisResult<CustomResult> = TxId<Chain, CustomResult>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        Ok(())
    }

    fn assert(self) -> Result<Self::AnalysisResult<Empty>, InterchainError> {
        Ok(self)
    }

    fn assert_custom<CustomResult>(
        self,
        _funcs: fn(&Binary) -> Result<CustomResult, InterchainError>,
    ) -> Result<Self::AnalysisResult<CustomResult>, InterchainError> {
        Ok(TxId::new(self.chain_id, self.response))
    }
}

impl<T: PacketAnalysis> PacketAnalysis for IbcPacketOutcome<T> {
    type AnalysisResult<CustomResult> =
        IbcPacketResult<T::AnalysisResult<CustomResult>, CustomResult>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        match &self {
            IbcPacketOutcome::Success { .. } => Ok(()),
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }

    fn assert(self) -> Result<Self::AnalysisResult<Empty>, InterchainError> {
        match self {
            IbcPacketOutcome::Success {
                receive_tx,
                ack_tx,
                ack,
            } => {
                let ibc_app_result = IbcAckParser::any_standard_app_result(&ack)?;

                // In case we have a polytone result, we verify that the execution result is not a failure
                if let crate::IbcAppResult::Polytone(_) = ibc_app_result {
                    // An execution failure in the ack for polytone, has those events :
                    // "method" --> "reply_callback_error"
                    // "packet_sequence" --> <packet-sequence>
                    // "callback_error" --> <callback-error>  to
                    let callback_error = ack_tx
                        .events()
                        .into_iter()
                        .filter(|e| e.ty == "wasm")
                        .filter_map(|e| {
                            let method_position = e.attributes.iter().position(|a| {
                                a.key == "method" && a.value == "reply_callback_error"
                            })?;
                            if let Some(packet_sequence) = e.attributes.get(method_position + 1) {
                                if packet_sequence.key != "packet_sequence" {
                                    return None;
                                }
                            } else {
                                return None;
                            }
                            if let Some(packet_sequence) = e.attributes.get(method_position + 2) {
                                if packet_sequence.key != "callback_error" {
                                    None
                                } else {
                                    Some(packet_sequence.value.clone())
                                }
                            } else {
                                None
                            }
                        })
                        .next();

                    if let Some(callback_error) = callback_error {
                        return Err(InterchainError::CallbackError(callback_error));
                    }
                };

                Ok(IbcPacketResult {
                    receive_tx: receive_tx.assert()?,
                    ack_tx: ack_tx.assert()?,
                    ibc_app_result,
                })
            }
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }

    fn assert_custom<CustomResult>(
        self,
        parsing_func: fn(&Binary) -> Result<CustomResult, InterchainError>,
    ) -> Result<Self::AnalysisResult<CustomResult>, InterchainError> {
        match self {
            IbcPacketOutcome::Success {
                receive_tx,
                ack_tx,
                ack,
            } => {
                let ibc_app_result =
                    IbcAckParser::any_standard_app_result_with_custom(&ack, parsing_func)?;
                Ok(IbcPacketResult {
                    receive_tx: receive_tx.assert_custom(parsing_func)?,
                    ack_tx: ack_tx.assert_custom(parsing_func)?,
                    ibc_app_result,
                })
            }
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }
}
impl<Chain: CwEnv> PacketAnalysis for SinglePacketFlow<Chain> {
    type AnalysisResult<CustomResult> = SuccessSinglePacketFlow<Chain, CustomResult>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        self.outcome.assert_no_timeout()
    }

    fn assert(self) -> Result<Self::AnalysisResult<Empty>, InterchainError> {
        let success = self.outcome.assert()?;

        Ok(SuccessSinglePacketFlow {
            send_tx: self.send_tx,
            result: success,
        })
    }

    fn assert_custom<CustomResult>(
        self,
        parse_func: fn(&Binary) -> Result<CustomResult, InterchainError>,
    ) -> Result<Self::AnalysisResult<CustomResult>, InterchainError> {
        let success = self.outcome.assert_custom(parse_func)?;

        Ok(SuccessSinglePacketFlow::<Chain, CustomResult> {
            send_tx: self.send_tx,
            result: success,
        })
    }
}

impl<Chain: CwEnv> PacketAnalysis for NestedPacketsFlow<Chain> {
    type AnalysisResult<CustomResult> = SuccessNestedPacketsFlow<Chain, CustomResult>;

    fn assert_no_timeout(&self) -> Result<(), InterchainError> {
        self.packets
            .iter()
            .map(|p| p.assert_no_timeout())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn assert(self) -> Result<Self::AnalysisResult<Empty>, InterchainError> {
        let packets = self
            .packets
            .into_iter()
            .map(|p| p.assert())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SuccessNestedPacketsFlow {
            tx_id: self.tx_id,
            packets,
        })
    }

    fn assert_custom<CustomResult>(
        self,
        parse_func: fn(&Binary) -> Result<CustomResult, InterchainError>,
    ) -> Result<Self::AnalysisResult<CustomResult>, InterchainError> {
        let packets = self
            .packets
            .into_iter()
            .map(|p| p.assert_custom(parse_func))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SuccessNestedPacketsFlow {
            tx_id: self.tx_id,
            packets,
        })
    }
}
