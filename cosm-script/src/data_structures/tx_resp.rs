use crate::cosmos_modules::abci::{AbciMessageLog, Attribute, StringEvent, TxResponse};
use crate::cosmos_modules::tendermint_abci::Event;
use crate::error::CosmScriptError;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";
const FORMAT_TZ_SUPPLIED: &str = "%Y-%m-%dT%H:%M:%S.%f%:z";
const FORMAT_SHORT_Z: &str = "%Y-%m-%dT%H:%M:%SZ";
const FORMAT_SHORT_Z2: &str = "%Y-%m-%dT%H:%M:%S.%fZ";
#[derive(Debug)]
pub struct CosmTxResponse {
    pub height: u64,
    pub txhash: String,
    pub codespace: String,
    pub code: usize,
    pub data: String,
    pub raw_log: String,
    pub logs: Vec<TxResultBlockMsg>,
    pub info: String,
    pub gas_wanted: u64,
    pub gas_used: u64,
    // pub tx: serde_json::Value,
    pub timestamp: DateTime<Utc>,
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
            //      log::info!("logs{:?}", events);
            let events_filtered = events
                .iter()
                .filter(|event| event.s_type == event_type)
                .collect::<Vec<_>>();
            //      log::info!("Filtered Events {:?}", events_filtered);
            if let Some(event) = events_filtered.first() {
                let attributes_filtered = event
                    .attributes
                    .iter()
                    .filter(|attr| attr.key == attribute_key)
                    .map(|f| Some(f.value.clone()))
                    .flatten()
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
            //      log::info!("logs{:?}", events);
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TxResultBlockAttribute {
    pub key: String,
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
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct TxResultBlockEvent {
    #[serde(rename = "type")]
    pub s_type: String,
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
    pub fn get_attribute(&self, key: &str) -> Vec<TxResultBlockAttribute> {
        self.attributes
            .iter()
            .filter(|attr| attr.key == key)
            .cloned()
            .collect()
    }
    /// return the first value of the first attribute that has the key 'key'
    pub fn get_first_value(&self, key: &str) -> Option<String> {
        self.get_attribute(key)
            .first()
            .map(|attr| attr.value.clone())
    }
}

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct TxResultBlockMsg {
    pub msg_index: Option<usize>,
    // pub log: Option<String>,
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

pub fn parse_timestamp(s: String) -> Result<DateTime<Utc>, CosmScriptError> {
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
                        eprintln!("DateTime Fail {} {:#?}", s, _e4);
                        Err(CosmScriptError::StdErr(_e4.to_string()))
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
