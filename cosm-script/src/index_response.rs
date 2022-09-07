use crate::{CosmTxResponse};
use cosmwasm_std::{Event, Binary};
use cw_multi_test::AppResponse;

// Function to index data returned by transactions which are applicable to both AppResponse (mock env) and TxResponse (live env)
pub trait IndexResponse {
    fn events(&self) -> Vec<Event>;
    fn event_attr_value(&self, event_type: &str, attr_key: &str) -> String;
    fn data(&self) -> Option<Binary>;
}

impl IndexResponse for AppResponse {
    fn events(&self) -> Vec<Event> {
        self.events
    }
    fn data(&self) -> Option<Binary> {
        self.data
    }

    fn event_attr_value(&self, event_type: &str, attr_key: &str) -> String {
        for event in self.events {
            if event.ty == event_type {
                for attr in event.attributes {
                    if attr.key == attr_key {
                        return attr.value
                    }
                }
            }
        }
    }
}

impl IndexResponse for CosmTxResponse {
    fn events(&self) -> Vec<Event> {
        self.events
    }
    fn data(&self) -> Option<Binary> {
        self.data
    }

    fn event_attr_value(&self, event_type: &str, attr_key: &str) -> String {
        for event in self.events {
            if event.ty == event_type {
                for attr in event.attributes {
                    if attr.key == attr_key {
                        return attr.value
                    }
                }
            }
        }
    }
}