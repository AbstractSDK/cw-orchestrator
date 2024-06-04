#![allow(non_snake_case)]

use cosmrs::{proto::traits::Name, tx::Msg, ErrorReport, Result};
/// MsgTransfer defines a msg to transfer fungible tokens (i.e Coins) between
/// ICS20 enabled chains. See ICS Spec here:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoMsgTransfer {
    /// the port on which the packet will be sent
    #[prost(string, tag = "1")]
    pub source_port: ::prost::alloc::string::String,
    /// the channel by which the packet will be sent
    #[prost(string, tag = "2")]
    pub source_channel: ::prost::alloc::string::String,
    /// the tokens to be transferred
    #[prost(message, optional, tag = "3")]
    pub token: ::core::option::Option<cosmrs::proto::cosmos::base::v1beta1::Coin>,
    /// the sender address
    #[prost(string, tag = "4")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag = "5")]
    pub receiver: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    #[prost(message, optional, tag = "6")]
    pub timeout_height: ::core::option::Option<cosmrs::proto::ibc::core::client::v1::Height>,
    /// Timeout timestamp in absolute nanoseconds since unix epoch.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag = "7")]
    pub timeout_timestamp: u64,
    /// Optional memo
    /// whole reason we are copying this from its original (proto::ibc::applications::transfer::v1::MsgTransfer)
    #[prost(string, optional, tag = "8")]
    pub memo: ::core::option::Option<::prost::alloc::string::String>,
}

impl Name for ProtoMsgTransfer {
    const NAME: &'static str = cosmrs::proto::ibc::applications::transfer::v1::MsgTransfer::NAME;
    const PACKAGE: &'static str =
        cosmrs::proto::ibc::applications::transfer::v1::MsgTransfer::PACKAGE;

    fn full_name() -> String {
        cosmrs::proto::ibc::applications::transfer::v1::MsgTransfer::full_name()
    }
}

/// MsgSend represents a message to send coins from one account to another.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgTransfer {
    /// Sender's address.
    pub source_port: String,
    pub source_channel: String,
    pub token: Option<cosmrs::Coin>,
    pub sender: cosmrs::AccountId,
    pub receiver: cosmrs::AccountId,
    pub timeout_height: Option<cosmrs::tendermint::block::Height>,
    pub timeout_revision: Option<u64>,
    pub timeout_timestamp: u64,
    pub memo: Option<String>,
}

impl Msg for MsgTransfer {
    type Proto = ProtoMsgTransfer;
}

impl TryFrom<ProtoMsgTransfer> for MsgTransfer {
    type Error = ErrorReport;

    fn try_from(proto: ProtoMsgTransfer) -> Result<MsgTransfer> {
        MsgTransfer::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgTransfer> for MsgTransfer {
    type Error = ErrorReport;

    fn try_from(proto: &ProtoMsgTransfer) -> Result<MsgTransfer> {
        Ok(MsgTransfer {
            source_port: proto.source_port.parse()?,
            source_channel: proto.source_channel.parse()?,
            token: proto.token.clone().map(TryFrom::try_from).transpose()?,
            sender: proto.sender.parse()?,
            receiver: proto.receiver.parse()?,
            timeout_height: proto
                .timeout_height
                .clone()
                .map(|h| h.revision_height.try_into())
                .transpose()?,
            timeout_revision: proto.timeout_height.clone().map(|h| h.revision_number),
            timeout_timestamp: proto.timeout_timestamp,
            memo: proto.memo.clone(),
        })
    }
}

impl From<MsgTransfer> for ProtoMsgTransfer {
    fn from(coin: MsgTransfer) -> ProtoMsgTransfer {
        ProtoMsgTransfer::from(&coin)
    }
}

impl From<&MsgTransfer> for ProtoMsgTransfer {
    fn from(msg: &MsgTransfer) -> ProtoMsgTransfer {
        ProtoMsgTransfer {
            source_port: msg.source_port.clone(),
            source_channel: msg.source_channel.clone(),
            token: msg.token.clone().map(Into::into),
            sender: msg.sender.to_string(),
            receiver: msg.receiver.to_string(),
            timeout_height: msg.timeout_height.map(|h| {
                cosmrs::proto::ibc::core::client::v1::Height {
                    revision_number: msg.timeout_revision.unwrap(),
                    revision_height: h.value(),
                }
            }),
            timeout_timestamp: msg.timeout_timestamp,
            memo: msg.memo.clone(),
        }
    }
}

// Tests
#[cfg(test)]
mod test {

    use std::time::{SystemTime, UNIX_EPOCH};

    use anyhow::Result as AnyResult;
    use cosmwasm_std::coin;
    use cw_orch_core::environment::TxHandler;

    use crate::tokenfactory::{
        create_denom, create_transfer_channel, get_denom, mint, transfer_tokens,
    };
    use cw_orch_interchain_core::{
        channel::InterchainChannel, types::IbcPacketOutcome, IbcQueryHandler, InterchainEnv,
    };
    use cw_orch_interchain_daemon::ChannelCreator;
    use cw_orch_starship::Starship;
    use cw_orch_traits::FullNode;
    use speculoos::{assert_that, vec::VecAssertions};
    use tokio::runtime::Runtime;

    const JUNO: &str = "juno-1";
    const STARGAZE: &str = "stargaze-1";

    const TEST_AMOUNT: u128 = 100_000_000_000;
    const TEST_SUBDENOM: &str = "testtoken";

    /// This allows env_logger to start properly for tests
    /// The logs will be printed only if the test fails !
    pub fn logger_test_init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    pub fn create_ics20_channel<Chain: IbcQueryHandler + FullNode, IBC: InterchainEnv<Chain>>(
        interchain: &IBC,
        chain_id1: &str,
        chain_id2: &str,
    ) -> AnyResult<(
        InterchainChannel<<Chain as IbcQueryHandler>::Handler>,
        String,
    )> {
        let chain1 = interchain.chain(chain_id1).unwrap();

        let sender = chain1.sender().to_string();

        let token_subdenom = format!(
            "{}{}",
            TEST_SUBDENOM,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        // Create Denom
        create_denom(&chain1, token_subdenom.as_str()).unwrap();

        // Mint Denom
        mint(
            &chain1,
            sender.as_str(),
            token_subdenom.as_str(),
            TEST_AMOUNT,
        )
        .unwrap();

        // Create a channel between the 2 chains for the transfer ports
        let interchain_channel = create_transfer_channel(chain_id1, chain_id2, interchain).unwrap();

        let denom = get_denom(&chain1, &token_subdenom);
        Ok((interchain_channel, denom))
    }

    #[ignore]
    #[test]
    pub fn create_ics20_channel_test() -> AnyResult<()> {
        logger_test_init();

        let rt = Runtime::new().unwrap();

        let starship = Starship::new(rt.handle(), None).unwrap();
        let interchain = starship.interchain_env();

        create_ics20_channel(&interchain, JUNO, STARGAZE)?;

        Ok(())
    }

    #[ignore]
    #[test]
    pub fn transfer_ics20_test() -> AnyResult<()> {
        logger_test_init();

        let rt = Runtime::new().unwrap();

        let starship = Starship::new(rt.handle(), None).unwrap();
        let interchain = starship.interchain_env();
        let (interchain_channel, denom) = create_ics20_channel(&interchain, JUNO, STARGAZE)?;

        let chain1 = starship.daemon(JUNO)?;
        let chain2 = starship.daemon(STARGAZE)?;

        // This should pass ok, the timeout was set right
        let success_outcome = transfer_tokens(
            chain1,
            chain2.sender().as_str(),
            &coin(TEST_AMOUNT / 2, denom.clone()),
            &interchain,
            &interchain_channel,
            None,
            None,
        )?;

        // We assert we had a success_outcome
        assert_that!(success_outcome.packets).has_length(1);

        // We want to assert the acknowledgment
        success_outcome.into_result()?;

        // This should timeout
        let timeout_outcome = transfer_tokens(
            chain1,
            chain2.sender().as_str(),
            &coin(TEST_AMOUNT / 2, denom),
            &interchain,
            &interchain_channel,
            Some(1),
            None,
        )?;

        // We assert we had a timeout_outcome
        assert_that!(timeout_outcome.packets).has_length(1);

        let packet_outcome = timeout_outcome.packets[0].outcome.clone();

        // We want to assert the acknowledgment
        match packet_outcome {
            IbcPacketOutcome::Timeout { .. } => {}
            _ => panic!("Wrong packet outcome"),
        };

        Ok(())
    }
}
