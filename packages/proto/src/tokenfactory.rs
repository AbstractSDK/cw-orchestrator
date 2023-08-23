#![allow(non_snake_case)]

use cosmrs::{tx::Msg, AccountId, ErrorReport, Result};

use cosmos_sdk_proto::traits::TypeUrl;

// This struct is an exact copy of https://github.com/osmosis-labs/osmosis-rust/blob/5997b8797a3210df0b1ab017025506a7645ff961/packages/osmosis-std/src/types/osmosis/tokenfactory/v1beta1.rs#L231
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoMsgCreateDenom {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// subdenom can be up to 44 "alphanumeric" characters long.
    #[prost(string, tag = "2")]
    pub subdenom: ::prost::alloc::string::String,
}

impl TypeUrl for ProtoMsgCreateDenom {
    const TYPE_URL: &'static str = "/osmosis.tokenfactory.v1beta1.MsgCreateDenom";
}

// This struct is an exact copy of https://github.com/osmosis-labs/osmosis-rust/blob/5997b8797a3210df0b1ab017025506a7645ff961/packages/osmosis-std/src/types/osmosis/tokenfactory/v1beta1.rs#L231
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoMsgMint {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub mint_to_address: ::prost::alloc::string::String,
}

impl TypeUrl for ProtoMsgMint {
    const TYPE_URL: &'static str = "/osmosis.tokenfactory.v1beta1.MsgMint";
}

/// MsgCreateDenom represents a message to send coins from one account to another.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgCreateDenom {
    /// Sender's address.
    pub sender: AccountId,

    /// Subdenom name
    pub subdenom: String,
}

impl TryFrom<ProtoMsgCreateDenom> for MsgCreateDenom {
    type Error = ErrorReport;

    fn try_from(proto: ProtoMsgCreateDenom) -> Result<MsgCreateDenom> {
        MsgCreateDenom::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgCreateDenom> for MsgCreateDenom {
    type Error = ErrorReport;

    fn try_from(proto: &ProtoMsgCreateDenom) -> Result<MsgCreateDenom> {
        Ok(MsgCreateDenom {
            sender: proto.sender.parse()?,
            subdenom: proto.subdenom.parse()?,
        })
    }
}

impl From<MsgCreateDenom> for ProtoMsgCreateDenom {
    fn from(coin: MsgCreateDenom) -> ProtoMsgCreateDenom {
        ProtoMsgCreateDenom::from(&coin)
    }
}

impl From<&MsgCreateDenom> for ProtoMsgCreateDenom {
    fn from(msg: &MsgCreateDenom) -> ProtoMsgCreateDenom {
        ProtoMsgCreateDenom {
            sender: msg.sender.to_string(),
            subdenom: msg.subdenom.to_string(),
        }
    }
}

impl Msg for MsgCreateDenom {
    type Proto = ProtoMsgCreateDenom;
}

/// MsgMint represents a message to send coins from one account to another.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgMint {
    /// Sender's address.
    pub sender: AccountId,

    /// Amount to mint
    pub amount: Option<cosmrs::Coin>,

    /// Recipient
    pub mint_to_address: AccountId,
}

impl TryFrom<ProtoMsgMint> for MsgMint {
    type Error = ErrorReport;

    fn try_from(proto: ProtoMsgMint) -> Result<MsgMint> {
        MsgMint::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgMint> for MsgMint {
    type Error = ErrorReport;

    fn try_from(proto: &ProtoMsgMint) -> Result<MsgMint> {
        Ok(MsgMint {
            sender: proto.sender.parse()?,
            amount: proto.amount.clone().map(TryFrom::try_from).transpose()?,
            mint_to_address: proto.mint_to_address.parse()?,
        })
    }
}

impl From<MsgMint> for ProtoMsgMint {
    fn from(coin: MsgMint) -> ProtoMsgMint {
        ProtoMsgMint::from(&coin)
    }
}

impl From<&MsgMint> for ProtoMsgMint {
    fn from(msg: &MsgMint) -> ProtoMsgMint {
        ProtoMsgMint {
            sender: msg.sender.to_string(),
            amount: msg.amount.clone().map(Into::into),
            mint_to_address: msg.mint_to_address.to_string(),
        }
    }
}

impl Msg for MsgMint {
    type Proto = ProtoMsgMint;
}

use std::str::FromStr;

use anyhow::Result as AnyResult;
use cosmrs::Denom;
use cosmwasm_std::Coin;
use cw_orch::{
    daemon::DaemonError,
    interchain::{interchain_channel::InterchainChannel, types::TxHashIbcAnalysisResult, IcResult},
    prelude::{interchain_channel_builder::InterchainChannelBuilder, Daemon, TxHandler},
    starship::Starship,
    state::ChainState,
};
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use tokio::runtime::Runtime;

use crate::ibc::MsgTransfer;

/// Creates a new denom using the token factory module.
/// This is used mainly for tests, but feel free to use that in production as well
pub async fn create_denom(daemon: &Daemon, token_name: &str) -> Result<(), DaemonError> {
    let creator = daemon.sender().to_string();
    daemon
        .wallet()
        .commit_tx(
            vec![MsgCreateDenom {
                sender: AccountId::from_str(creator.as_str())?,
                subdenom: token_name.to_string(),
            }],
            None,
        )
        .await?;

    log::info!("Created denom {}", get_denom(daemon, token_name));

    Ok(())
}

/// Gets the denom of a token created by a daemon object
/// This actually creates the denom for a token created by an address (which is here taken to be the daemon sender address)
/// This is mainly used for tests, but feel free to use that in production as well
pub fn get_denom(daemon: &Daemon, token_name: &str) -> String {
    let sender = daemon.sender().to_string();
    format!("factory/{}/{}", sender, token_name)
}

/// Mints new subdenom token for which the minter is the sender of Daemon object
/// This mints new tokens to the receiver address
/// This is mainly used for tests, but feel free to use that in production as well
pub async fn mint(
    daemon: &Daemon,
    receiver: &str,
    token_name: &str,
    amount: u128,
) -> Result<(), DaemonError> {
    let sender = daemon.sender().to_string();
    let denom = get_denom(daemon, token_name);

    daemon
        .wallet()
        .commit_tx(
            vec![MsgMint {
                sender: AccountId::from_str(sender.as_str())?,
                mint_to_address: AccountId::from_str(receiver)?,
                amount: Some(cosmrs::Coin {
                    denom: Denom::from_str(denom.as_str())?,
                    amount,
                }),
            }],
            None,
        )
        .await?;

    log::info!("Minted coins {} {}", amount, get_denom(daemon, token_name));

    Ok(())
}

// 1 hour should be sufficient for packet timeout
const TIMEOUT_IN_NANO_SECONDS: u64 = 3_600_000_000_000;

/// Ibc token transfer
/// This allows transfering token over a channel using an interchain_channel object
pub fn transfer_tokens(
    rt: &Runtime,
    origin: &Daemon,
    receiver: &str,
    fund: &Coin,
    ibc_channel: &InterchainChannel,
    timeout: Option<u64>,
    memo: Option<String>,
) -> IcResult<TxHashIbcAnalysisResult> {
    let chain_id = origin.state().chain_data.chain_id.to_string();

    let source_port = ibc_channel.get_chain(chain_id)?;

    // We send tokens using the ics20 message over the channel that is passed as an argument
    let send_tx = rt.block_on(origin.wallet().commit_tx(
        vec![MsgTransfer {
            source_port: source_port.port.to_string(),
            source_channel: source_port.channel.unwrap().to_string(),
            token: Some(cosmrs::Coin {
                amount: fund.amount.u128(),
                denom: Denom::from_str(fund.denom.as_str()).unwrap(),
            }),
            sender: AccountId::from_str(origin.sender().to_string().as_str()).unwrap(),
            receiver: AccountId::from_str(receiver).unwrap(),
            timeout_height: None,
            timeout_revision: None,
            timeout_timestamp: origin.block_info()?.time.nanos()
                + timeout.unwrap_or(TIMEOUT_IN_NANO_SECONDS),
            memo,
        }],
        None,
    ))?;

    // We wait for the IBC tx to stop successfully
    let tx_results =
        rt.block_on(ibc_channel.await_ibc_execution(source_port.chain_id, send_tx.txhash))?;

    Ok(tx_results)
}

/* ####################### STARSHIP specific functions ########################### */

const ICS20_CHANNEL_VERSION: &str = "ics20-1";
/// Channel creation between the transfer channels of two blockchains of a starship integration
pub async fn create_transfer_channel(
    chain1: &str,
    chain2: &str,
    starship: &Starship,
) -> AnyResult<InterchainChannel> {
    let daemon_a = starship.daemon(chain1)?;
    let daemon_b = starship.daemon(chain2)?;
    Ok(InterchainChannelBuilder::default()
        .from_daemons(daemon_a, daemon_b)
        .port_a(PortId::transfer())
        .port_b(PortId::transfer())
        .create_channel(starship.client(), ICS20_CHANNEL_VERSION)
        .await?)
}
