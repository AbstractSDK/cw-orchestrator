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

#[cfg(test)]
mod index_response {
    use cosmwasm_std::{Event, Addr};
    use cw_multi_test::AppResponse;

    use speculoos::prelude::*;

    use super::IndexResponse;

    const CONTRACT_ADDRESS: &str = "cosmos1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr";

    fn test_events(idxres: &dyn IndexResponse) -> anyhow::Result<()> {
        asserting!("events length is 1")
            .that(&idxres.events().len())
            .is_equal_to(&2);

        Ok(())
    }

    fn test_data(idxres: &dyn IndexResponse) -> anyhow::Result<()> {
        asserting!("data is None")
            .that(&idxres.data())
            .is_none();

        Ok(())
    }

    fn test_uploaded_code_id(idxres: &dyn IndexResponse) -> anyhow::Result<()> {
        asserting!("uploaded code_id is 1")
            .that(&idxres.uploaded_code_id()?)
            .is_equal_to(&1u64);

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
                Event::new("instantiate").add_attribute("_contract_address", CONTRACT_ADDRESS.to_owned()),
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
