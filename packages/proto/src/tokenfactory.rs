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
