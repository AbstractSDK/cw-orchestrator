use std::str::FromStr;

use base64::Engine;
use color_eyre::eyre::Context;
use inquire::Select;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Debug, EnumDiscriminants, Clone, clap::ValueEnum)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// How do you want to pass the message arguments?
pub enum MsgType {
    #[strum_discriminants(strum(message = "json message"))]
    /// Valid JSON string (e.g. {"foo": "bar"})
    JsonMsg,
    #[strum_discriminants(strum(message = "base64 message"))]
    /// Base64-encoded string (e.g. eyJmb28iOiJiYXIifQ==)
    Base64Msg,
}

impl interactive_clap::ToCli for MsgType {
    type CliVariant = MsgType;
}

impl std::str::FromStr for MsgType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json-msg" => Ok(Self::JsonMsg),
            "base64-msg" => Ok(Self::Base64Msg),
            _ => Err("MsgType: incorrect message type".to_string()),
        }
    }
}

impl std::fmt::Display for MsgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JsonMsg => write!(f, "json-msg"),
            Self::Base64Msg => write!(f, "base64-msg"),
        }
    }
}

impl std::fmt::Display for MsgTypeDiscriminants {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JsonMsg => write!(f, "json-msg"),
            Self::Base64Msg => write!(f, "base64-msg"),
        }
    }
}

pub fn input_msg_type() -> color_eyre::eyre::Result<Option<MsgType>> {
    let variants = MsgTypeDiscriminants::iter().collect::<Vec<_>>();
    let selected = Select::new("Select message format", variants).prompt()?;
    match selected {
        MsgTypeDiscriminants::JsonMsg => Ok(Some(MsgType::JsonMsg)),
        MsgTypeDiscriminants::Base64Msg => Ok(Some(MsgType::Base64Msg)),
    }
}

pub fn msg_bytes(message: String, msg_type: MsgType) -> color_eyre::eyre::Result<Vec<u8>> {
    match msg_type {
        MsgType::JsonMsg => {
            let data_json =
                serde_json::Value::from_str(&message).wrap_err("Data not in JSON format!")?;
            Ok(data_json.to_string().into_bytes())
        }
        MsgType::Base64Msg => Ok(crate::common::B64.decode(&message)?),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_message() {
        let b_64_msg = msg_bytes(
            "eyJsYXRlc3RfY29udHJhY3RzIjp7fX0=".to_owned(),
            MsgType::Base64Msg,
        )
        .unwrap();
        let json_msg =
            msg_bytes(r#"{"latest_contracts":{}}"#.to_owned(), MsgType::JsonMsg).unwrap();

        assert_eq!(b_64_msg, json_msg);
    }
}
