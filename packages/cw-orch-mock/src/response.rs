use cosmwasm_std::{Attribute, Binary, Event, StdError, StdResult};
use cw_orch_core::environment::IndexResponse;

#[derive(Default, Debug, Clone)]
pub struct AppResponse {
    pub events: Vec<Event>,
    pub data: Option<Binary>,
}

impl AppResponse {
    /// Returns all custom attributes returned by the contract in the `idx` event.
    ///
    /// We assert the type is wasm, and skip the contract_address attribute.
    #[track_caller]
    pub fn custom_attrs(&self, idx: usize) -> &[Attribute] {
        assert_eq!(self.events[idx].ty.as_str(), "wasm");
        &self.events[idx].attributes[1..]
    }

    /// Checks if there is an Event that is a super-set of this.
    ///
    /// It has the same type, and all compare.attributes are included in it as well.
    /// You don't need to specify them all.
    pub fn has_event(&self, expected: &Event) -> bool {
        self.events.iter().any(|ev| {
            expected.ty == ev.ty
                && expected
                    .attributes
                    .iter()
                    .all(|at| ev.attributes.contains(at))
        })
    }

    /// Like [has_event](Self::has_event) but panics if there is no match.
    #[track_caller]
    pub fn assert_event(&self, expected: &Event) {
        assert!(
            self.has_event(expected),
            "Expected to find an event {:?}, but received: {:?}",
            expected,
            self.events
        );
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
        Err(StdError::generic_err(format!(
            "missing combination (event: {}, attribute: {})",
            event_type, attr_key
        )))
    }
}

impl From<cw_multi_test::AppResponse> for AppResponse {
    fn from(value: cw_multi_test::AppResponse) -> Self {
        AppResponse {
            events: value.events,
            data: value.data,
        }
    }
}

#[cfg(test)]
mod index_response_test {
    use cosmwasm_std::{Addr, Event};

    use speculoos::prelude::*;

    use crate::AppResponse;

    use super::IndexResponse;

    const CONTRACT_ADDRESS: &str =
        "cosmos1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr";

    fn test_events(idxres: &dyn IndexResponse) -> anyhow::Result<()> {
        asserting!("events length is 1")
            .that(&idxres.events().len())
            .is_equal_to(2);

        Ok(())
    }

    fn test_data(idxres: &dyn IndexResponse) -> anyhow::Result<()> {
        asserting!("data is None").that(&idxres.data()).is_none();

        Ok(())
    }

    fn test_uploaded_code_id(idxres: &dyn IndexResponse) -> anyhow::Result<()> {
        asserting!("uploaded code_id is 1")
            .that(&idxres.uploaded_code_id()?)
            .is_equal_to(1u64);

        Ok(())
    }

    fn test_instantiated_contract_address(idxres: &dyn IndexResponse) -> anyhow::Result<()> {
        asserting!("instantiated contract_address is ")
            .that(&idxres.instantiated_contract_address()?)
            .is_equal_to(&Addr::unchecked(CONTRACT_ADDRESS));

        Ok(())
    }

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
