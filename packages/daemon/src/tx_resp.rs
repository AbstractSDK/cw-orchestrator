use super::{
    cosmos_modules::{
        abci::{AbciMessageLog, Attribute, StringEvent, TxResponse},
        tendermint_abci::Event,
    },
    error::DaemonError,
};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use cosmwasm_std::{to_binary, Binary, StdError, StdResult};
use cw_orch_environment::environment::IndexResponse;
use serde::{Deserialize, Serialize};

const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";
const FORMAT_TZ_SUPPLIED: &str = "%Y-%m-%dT%H:%M:%S.%f%:z";
const FORMAT_SHORT_Z: &str = "%Y-%m-%dT%H:%M:%SZ";
const FORMAT_SHORT_Z2: &str = "%Y-%m-%dT%H:%M:%S.%fZ";

/// The response from a transaction performed on a blockchain.
#[derive(Debug, Clone, Default)]
pub struct CosmTxResponse {
    /// Height of the block in which the transaction was included.
    pub height: u64,
    /// Transaction hash.
    pub txhash: String,
    /// Transaction index within the block.
    pub codespace: String,
    /// Transaction result code
    pub code: usize,
    /// Arbitrary data that can be included in a transaction.
    pub data: String,
    /// Raw log message.
    pub raw_log: String,
    /// Logs of the transaction.
    pub logs: Vec<TxResultBlockMsg>,
    /// Transaction info.
    pub info: String,
    /// Gas limit.
    pub gas_wanted: u64,
    /// Gas used.
    pub gas_used: u64,
    /// Timestamp of the block in which the transaction was included.
    pub timestamp: DateTime<Utc>,
    /// Transaction events.
    pub events: Vec<Event>,
}

impl CosmTxResponse {
    /// find a attribute's value from TX logs.
    /// returns: msg_index and value
    pub fn get_attribute_from_logs(
        &self,
        event_type: &str,
        attribute_key: &str,
    ) -> Vec<(usize, String)> {
        let mut response: Vec<(usize, String)> = Default::default();
        let logs = &self.logs;

        for log_part in logs {
            let msg_index = log_part.msg_index.unwrap_or_default();
            let events = &log_part.events;

            let events_filtered = events
                .iter()
                .filter(|event| event.s_type == event_type)
                .collect::<Vec<_>>();

            if let Some(event) = events_filtered.first() {
                let attributes_filtered = event
                    .attributes
                    .iter()
                    .filter(|attr| attr.key == attribute_key)
                    .map(|f| f.value.clone())
                    .collect::<Vec<_>>();

                if let Some(attr_key) = attributes_filtered.first() {
                    response.push((msg_index, attr_key.clone()));
                }
            }
        }

        response
    }

    /// get the list of event types from a TX record
    pub fn get_events(&self, event_type: &str) -> Vec<TxResultBlockEvent> {
        let mut response: Vec<TxResultBlockEvent> = Default::default();

        for log_part in &self.logs {
            let events = &log_part.events;

            let events_filtered = events
                .iter()
                .filter(|event| event.s_type == event_type)
                .collect::<Vec<_>>();

            for event in events_filtered {
                response.push(event.clone());
            }
        }

        response
    }
}

// NOTE: Should we keep this here or only for tests?
impl From<&serde_json::Value> for TxResultBlockMsg {
    fn from(value: &serde_json::Value) -> Self {
        serde_json::from_value(value.clone()).unwrap()
    }
}

impl From<TxResponse> for CosmTxResponse {
    fn from(tx: TxResponse) -> Self {
        Self {
            height: tx.height as u64,
            txhash: tx.txhash,
            codespace: tx.codespace,
            code: tx.code as usize,
            data: tx.data,
            raw_log: tx.raw_log,
            logs: tx.logs.into_iter().map(TxResultBlockMsg::from).collect(),
            info: tx.info,
            gas_wanted: tx.gas_wanted as u64,
            gas_used: tx.gas_used as u64,
            timestamp: parse_timestamp(tx.timestamp).unwrap(),
            events: tx.events,
        }
    }
}

impl IndexResponse for CosmTxResponse {
    fn events(&self) -> Vec<cosmwasm_std::Event> {
        let mut parsed_events = vec![];

        for event in &self.events {
            let mut pattr = vec![];

            for attr in &event.attributes {
                pattr.push(cosmwasm_std::Attribute {
                    key: attr.key.clone(),
                    value: attr.value.clone(),
                })
            }

            let pevent = cosmwasm_std::Event::new(event.r#type.clone()).add_attributes(pattr);

            parsed_events.push(pevent);
        }

        parsed_events
    }

    fn data(&self) -> Option<Binary> {
        if self.data.is_empty() {
            None
        } else {
            Some(to_binary(self.data.as_bytes()).unwrap())
        }
    }

    fn event_attr_value(&self, event_type: &str, attr_key: &str) -> StdResult<String> {
        for event in &self.events {
            if event.r#type == event_type {
                for attr in &event.attributes {
                    if attr.key == attr_key {
                        return Ok(attr.value.clone());
                    }
                }
            }
        }

        Err(StdError::generic_err(format!(
            "event of type {event_type} does not have a value at key {attr_key}"
        )))
    }
}

/// The events from a single message in a transaction.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TxResultBlockMsg {
    /// index of the message in the transaction
    pub msg_index: Option<usize>,
    /// Events from this message
    pub events: Vec<TxResultBlockEvent>,
}

impl From<AbciMessageLog> for TxResultBlockMsg {
    fn from(msg: AbciMessageLog) -> Self {
        Self {
            msg_index: Some(msg.msg_index as usize),
            events: msg
                .events
                .into_iter()
                .map(TxResultBlockEvent::from)
                .collect(),
        }
    }
}

/// A single event from a transaction and its attributes.
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct TxResultBlockEvent {
    #[serde(rename = "type")]
    /// Type of the event
    pub s_type: String,
    /// Attributes of the event
    pub attributes: Vec<TxResultBlockAttribute>,
}

impl From<StringEvent> for TxResultBlockEvent {
    fn from(event: StringEvent) -> Self {
        Self {
            s_type: event.r#type,
            attributes: event
                .attributes
                .into_iter()
                .map(TxResultBlockAttribute::from)
                .collect(),
        }
    }
}

impl TxResultBlockEvent {
    /// get all key/values from the event that have the key 'key'
    pub fn get_attributes(&self, key: &str) -> Vec<TxResultBlockAttribute> {
        self.attributes
            .iter()
            .filter(|attr| attr.key == key)
            .cloned()
            .collect()
    }

    /// return the first value of the first attribute that has the key 'key'
    pub fn get_first_attribute_value(&self, key: &str) -> Option<String> {
        self.get_attributes(key)
            .first()
            .map(|attr| attr.value.clone())
    }
}

/// A single attribute of an event.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TxResultBlockAttribute {
    /// Key of the attribute
    pub key: String,
    /// Value of the attribute
    pub value: String,
}

impl From<Attribute> for TxResultBlockAttribute {
    fn from(a: Attribute) -> Self {
        Self {
            key: a.key,
            value: a.value,
        }
    }
}

/// Parse a string timestamp into a `DateTime<Utc>`
pub fn parse_timestamp(s: String) -> Result<DateTime<Utc>, DaemonError> {
    let len = s.len();

    let slice_len = if s.contains('.') {
        len.saturating_sub(4)
    } else {
        len
    };

    let sliced = &s[0..slice_len];

    match NaiveDateTime::parse_from_str(sliced, FORMAT) {
        Err(_e) => match NaiveDateTime::parse_from_str(&s, FORMAT_TZ_SUPPLIED) {
            Err(_e2) => match NaiveDateTime::parse_from_str(sliced, FORMAT_SHORT_Z) {
                // block 6877827 has this
                Err(_e3) => match NaiveDateTime::parse_from_str(&s, FORMAT_SHORT_Z2) {
                    Err(_e4) => {
                        eprintln!("DateTime Fail {s} {_e4:#?}");
                        Err(DaemonError::StdErr(_e4.to_string()))
                    }
                    Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
                },
                Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
            },
            Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
        },
        Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
    }
}
