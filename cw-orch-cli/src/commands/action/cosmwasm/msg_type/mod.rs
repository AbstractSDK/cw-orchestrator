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
        MsgTypeDiscriminants::Editor => Ok(Some(MsgType::File)),
    }
}

pub fn input_msg_or_filename() -> color_eyre::eyre::Result<Option<String>> {
    let input = inquire::Text::new("Enter message or filename")
        .with_help_message("Leave non-file message input empty for EDITOR input later")
        .prompt()?;
    Ok(Some(input))
}

pub fn msg_bytes(message_or_file: String, msg_type: MsgType) -> color_eyre::eyre::Result<Vec<u8>> {
    match msg_type {
        MsgType::JsonMsg => {
            let message_json = serde_json::Value::from_str(&message_or_file)
                .wrap_err("Message not in JSON format")?;

            serde_json::to_vec(&message_json).wrap_err("Unexpected error")
        }
        MsgType::Base64Msg => {
            let message = match message_or_file.is_empty() {
                false => message_or_file,
                true => inquire::Editor::new("Enter")
                    .with_help_message("Base64-encoded string (e.g. eyJmb28iOiJiYXIifQ==)")
                    .prompt()?,
            };

            crate::common::B64
                .decode(message)
                .wrap_err("Failed to decode base64 string")
        }
        MsgType::File => {
            let file_path = std::path::PathBuf::from(message_or_file);
            let msg_bytes =
                std::fs::read(file_path.as_path()).wrap_err("Failed to read a message file")?;
            Ok(msg_bytes)
        }
        MsgType::Editor => {
            let message = inquire::Editor::new("Enter message")
                .with_predefined_text("{}")
                .with_file_extension(".json")
                .prompt()?;
            Ok(message.into_bytes())
        }
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
