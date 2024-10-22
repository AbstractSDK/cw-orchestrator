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
    #[strum_discriminants(strum(message = "Read from a file"))]
    /// Read from a file (e.g. file.json)
    File,
    #[strum_discriminants(strum(message = "Use your editor"))]
    /// Open editor (uses EDITOR env variable) to input message
    Editor,
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
            "file" => Ok(Self::File),
            "editor" => Ok(Self::Editor),
            _ => Err("MsgType: incorrect message type".to_string()),
        }
    }
}

impl std::fmt::Display for MsgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JsonMsg => write!(f, "json-msg"),
            Self::Base64Msg => write!(f, "base64-msg"),
            Self::File => write!(f, "file"),
            Self::Editor => write!(f, "editor"),
        }
    }
}

impl std::fmt::Display for MsgTypeDiscriminants {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JsonMsg => write!(f, "Json Msg"),
            Self::Base64Msg => write!(f, "Base64 Msg"),
            Self::File => write!(f, "File"),
            Self::Editor => write!(f, "Editor"),
        }
    }
}

pub fn input_msg_type() -> color_eyre::eyre::Result<Option<MsgType>> {
    let variants = MsgTypeDiscriminants::iter().collect::<Vec<_>>();
    let selected = Select::new("Select message format", variants).prompt()?;
    match selected {
        MsgTypeDiscriminants::JsonMsg => Ok(Some(MsgType::JsonMsg)),
        MsgTypeDiscriminants::Base64Msg => Ok(Some(MsgType::Base64Msg)),
        MsgTypeDiscriminants::File => Ok(Some(MsgType::File)),
        MsgTypeDiscriminants::Editor => Ok(Some(MsgType::Editor)),
    }
}

pub fn msg_bytes(message_or_file: String, msg_type: MsgType) -> color_eyre::eyre::Result<Vec<u8>> {
    match msg_type {
        MsgType::JsonMsg => {
            let message_json = serde_json::Value::from_str(&message_or_file)
                .wrap_err("Message not in JSON format")?;

            serde_json::to_vec(&message_json).wrap_err("Unexpected error")
        }
        MsgType::Base64Msg => crate::common::B64
            .decode(message_or_file)
            .wrap_err("Failed to decode base64 string"),
        MsgType::File => {
            let file_path = std::path::PathBuf::from(message_or_file);
            let msg_bytes =
                std::fs::read(file_path.as_path()).wrap_err("Failed to read a message file")?;
            Ok(msg_bytes)
        }
        MsgType::Editor => {
            let mut prompt = inquire::Editor::new("Enter message");
            if message_or_file.is_empty() {
                prompt = prompt
                    .with_predefined_text("{}")
                    .with_file_extension(".json");
            } else {
                prompt = prompt.with_file_extension(&message_or_file)
            };
            let message = prompt.prompt()?;
            Ok(message.into_bytes())
        }
    }
}

#[derive(Debug, EnumDiscriminants, Clone, Copy, clap::ValueEnum)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// How do you want to pass the key arguments?
pub enum KeyType {
    #[strum_discriminants(strum(message = "Raw string"))]
    /// Raw string (e.g. contract_info)
    Raw,
    #[strum_discriminants(strum(message = "base64 message"))]
    /// Base64-encoded string (e.g. Y29udHJhY3QtaW5mbw==)
    Base64,
}

impl interactive_clap::ToCli for KeyType {
    type CliVariant = KeyType;
}

impl std::str::FromStr for KeyType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "raw" => Ok(Self::Raw),
            "base64" => Ok(Self::Base64),
            _ => Err("KeyType: incorrect key type".to_string()),
        }
    }
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Raw => write!(f, "raw"),
            Self::Base64 => write!(f, "base64"),
        }
    }
}

impl std::fmt::Display for KeyTypeDiscriminants {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Raw => write!(f, "Raw"),
            Self::Base64 => write!(f, "Base64"),
        }
    }
}

pub fn input_key_type() -> color_eyre::eyre::Result<Option<KeyType>> {
    let variants = KeyTypeDiscriminants::iter().collect::<Vec<_>>();
    let selected = Select::new("Select key format", variants).prompt()?;
    match selected {
        KeyTypeDiscriminants::Raw => Ok(Some(KeyType::Raw)),
        KeyTypeDiscriminants::Base64 => Ok(Some(KeyType::Base64)),
    }
}

pub fn key_bytes(key: String, key_type: KeyType) -> color_eyre::eyre::Result<Vec<u8>> {
    match key_type {
        KeyType::Raw => Ok(key.into_bytes()),
        KeyType::Base64 => crate::common::B64
            .decode(key)
            .wrap_err("Failed to decode base64 string"),
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
