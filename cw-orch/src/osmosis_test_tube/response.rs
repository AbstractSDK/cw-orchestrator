use cosmwasm_std::{Binary, Event, StdError, StdResult};
use cw_orch_core::environment::IndexResponse;

#[derive(Default, Debug, Clone)]
pub struct AppResponse {
    pub events: Vec<Event>,
    pub data: Option<Binary>,
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
        Err(StdError::generic_err(format!(
            "missing combination (event: {}, attribute: {})",
            event_type, attr_key
        )))
    }
}

impl From<crate::mock::cw_multi_test::AppResponse> for AppResponse {
    fn from(value: crate::mock::cw_multi_test::AppResponse) -> Self {
        AppResponse {
            events: value.events,
            data: value.data,
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn general() {
        let idxres = AppResponse {
            events: vec![
                Event::new("store_code").add_attribute("code_id", "1"),
                Event::new("instantiate")
                    .add_attribute("_contract_address", CONTRACT_ADDRESS.to_owned()),
            ],
            data: None,
        };

        asserting!("test_events is ok")
            .that(&test_events(&idxres))
            .is_ok();

        asserting!("test_data is ok")
            .that(&test_data(&idxres))
            .is_ok();

        asserting!("test_instantiated_contract_address is ok")
            .that(&test_instantiated_contract_address(&idxres))
            .is_ok();

        asserting!("test_uploaded_code_id is ok")
            .that(&test_uploaded_code_id(&idxres))
            .is_ok();
    }
}
