use cosmwasm_schema::cw_serde;
use cosmwasm_std::{from_json, Binary};
use cw_orch_core::environment::CwEnv;
use polytone::ack::Callback;
use prost::Message;

use crate::{
    env::decode_ack_error,
    types::{parse::SuccessIbcPacket, IbcTxAnalysis},
    InterchainError,
};

use self::acknowledgement::{Acknowledgement, Response};

/// Struct used to centralize all the pre-defined ack types
pub enum IbcAckParser {}

impl IbcAckParser {
    /// Verifies if the given ack is an Polytone type and returns the acknowledgement if it is
    ///
    /// Returns an error if there was an error in the process
    pub fn polytone_ack(ack: &Binary) -> Result<Callback, InterchainError> {
        // We try decoding the ack with Polytone
        let decoded_polytone_packet: Result<Callback, _> = from_json(ack);
        if let Ok(decoded_polytone_packet) = decoded_polytone_packet {
            match &decoded_polytone_packet {
                Callback::Query(query_result) => match query_result {
                    Ok(_) => return Ok(decoded_polytone_packet),
                    Err(e) => {
                        return Err(InterchainError::FailedAckReceived(format!(
                            "Error during query on remote chain : {:?}",
                            e
                        )))
                    }
                },
                Callback::Execute(execute_response) => match execute_response {
                    Ok(_) => return Ok(decoded_polytone_packet),
                    Err(e) => {
                        return Err(InterchainError::FailedAckReceived(format!(
                            "Error during execution on remote chain : {}",
                            e
                        )))
                    }
                },
                Callback::FatalError(e) => {
                    return Err(InterchainError::FailedAckReceived(e.to_string()))
                }
            }
        }

        Err(decode_ack_error(ack))
    }

    /// Verifies if the given ack is an IBC20 type
    ///
    /// Returns an error if there was an error in the parsing process
    pub fn ics20_ack(ack: &Binary) -> Result<(), InterchainError> {
        let decoded_fungible_packet: Result<FungibleTokenPacketAcknowledgement, _> = from_json(ack);
        if let Ok(decoded_fungible_packet) = decoded_fungible_packet {
            match decoded_fungible_packet {
                FungibleTokenPacketAcknowledgement::Result(_) => return Ok(()),
                FungibleTokenPacketAcknowledgement::Error(e) => {
                    return Err(InterchainError::FailedAckReceived(e))
                }
            }
        }
        Err(decode_ack_error(ack))
    }

    /// Verifies if the given ack is an ICS004 type and returns the ack result if it is
    ///
    /// Returns an error if there was an error in the parsing process
    pub fn ics004_ack(ack: &Binary) -> Result<Vec<u8>, InterchainError> {
        if let Ok(decoded_ics_004) = Acknowledgement::decode(ack.as_slice()) {
            if let Some(response) = decoded_ics_004.response {
                log::debug!("Decoded ack using ICS-004 : {:x?}", response);
                match response {
                    Response::Result(result) => return Ok(result),
                    Response::Error(e) => return Err(InterchainError::FailedAckReceived(e)),
                }
            }
        }
        Err(decode_ack_error(ack))
    }
}

#[cw_serde]
/// Taken from https://github.com/cosmos/ibc/blob/main/spec/app/ics-020-fungible-token-transfer/README.md#data-structures
pub enum FungibleTokenPacketAcknowledgement {
    /// Successful packet
    Result(String),
    /// Error packet
    Error(String),
}

impl<Chain: CwEnv> IbcTxAnalysis<Chain> {
    /// Assert that all packets were not timeout
    pub fn assert_no_timeout(&self) -> Result<Vec<SuccessIbcPacket<Chain>>, InterchainError> {
        Ok(self
            .packets
            .iter()
            .map(|p| p.assert_no_timeout())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    /// Returns all packets that were successful without asserting there was no timeout
    pub fn get_success_packets(&self) -> Result<Vec<SuccessIbcPacket<Chain>>, InterchainError> {
        Ok(self
            .packets
            .iter()
            .map(|p| p.get_success_packets())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}

/// This is copied from https://github.com/cosmos/cosmos-rust/blob/4f2e3bbf9c67c8ffef44ef1e485a327fd66f060a/cosmos-sdk-proto/src/prost/ibc-go/ibc.core.channel.v1.rs#L164
/// This is the ICS-004 standard proposal
pub mod acknowledgement {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Acknowledgement {
        /// response contains either a result or an error and must be non-empty
        #[prost(oneof = "Response", tags = "21, 22")]
        pub response: ::core::option::Option<Response>,
    }
    /// response contains either a result or an error and must be non-empty
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(bytes, tag = "21")]
        Result(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "22")]
        Error(::prost::alloc::string::String),
    }

    impl ::prost::Name for Acknowledgement {
        const NAME: &'static str = "Acknowledgement";
        const PACKAGE: &'static str = "ibc.core.channel.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
        }
    }
}
