use crate::daemon::CosmTxResponse;
use crate::prelude::IndexResponse;

use cosmwasm_std::{to_binary, Binary, StdError, StdResult};

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
