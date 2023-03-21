use cosmwasm_std::{Addr, Binary, Event, StdError, StdResult};
use cw_multi_test::AppResponse;

/// Index data returned by transactions which are applicable to both AppResponse (mock env) and TxResponse (live env)
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
