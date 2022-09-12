use crate::CosmTxResponse;
use cosmwasm_std::{to_binary, Addr, Attribute, Binary, Event, StdError, StdResult};
use cw_multi_test::AppResponse;

// Function to index data returned by transactions which are applicable to both AppResponse (mock env) and TxResponse (live env)
pub trait IndexResponse {
    fn events(&self) -> Vec<Event>;
    fn event_attr_value(&self, event_type: &str, attr_key: &str) -> StdResult<String>;
    fn data(&self) -> Option<Binary>;
    fn instantiated_contract_address(&self) -> StdResult<Addr> {
        self.event_attr_value("instantiate", "_contract_address")
            .map(Addr::unchecked)
    }
    fn uploaded_code_id(&self) -> StdResult<u64> {
        self.event_attr_value("store_code", "code_id")
            .map(|s| s.parse().unwrap())
    }
}

impl IndexResponse for AppResponse {
    fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
    fn data(&self) -> Option<Binary> {
        self.data.clone()
    }

    fn event_attr_value(&self, event_type: &str, attr_key: &str) -> StdResult<String> {
        for event in &self.events {
            if event.ty == event_type {
                for attr in &event.attributes {
                    if attr.key == attr_key {
                        return Ok(attr.value.clone());
                    }
                }
            }
        }
        Err(StdError::generic_err("missing "))
    }
}

impl IndexResponse for CosmTxResponse {
    fn events(&self) -> Vec<Event> {
        let mut parsed_events = vec![];
        for event in &self.events {
            let mut pattr = vec![];
            for attr in &event.attributes {
                pattr.push(Attribute {
                    key: std::str::from_utf8(&attr.key).unwrap().to_string(),
                    value: std::str::from_utf8(&attr.value).unwrap().to_string(),
                })
            }
            let pevent = Event::new(event.r#type.clone()).add_attributes(pattr);
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
                    if attr.key == attr_key.as_bytes() {
                        return Ok(std::str::from_utf8(&attr.value).unwrap().to_string());
                    }
                }
            }
        }
        Err(StdError::generic_err(format!(
            "event of type {} does not have a value at key {}",
            event_type, attr_key
        )))
    }
}
