use cosmwasm_std::{Addr, Binary, Event, StdError, StdResult};
#[cfg(feature = "eth")]
use snailquote::unescape;

const CODE_ID_UPLOAD_EVENT: (&str, &str) = ("store_code", "code_id");
const ADDRESS_INSTANTIATE_EVENT: (&str, &str) = ("instantiate", "_contract_address");

#[cfg(feature = "eth")]
const INJECTIVE_CODE_ID_UPLOAD_EVENT: (&str, &str) =
    ("cosmwasm.wasm.v1.EventCodeStored", "code_id");
#[cfg(feature = "eth")]
const INJECTIVE_ADDRESS_INSTANTIATE_EVENT: (&str, &str) = (
    "cosmwasm.wasm.v1.EventContractInstantiated",
    "contract_address",
);

/// Index data returned by transactions which are applicable to both AppResponse (mock env) and TxResponse (live env)
pub trait IndexResponse {
    /// Get all events in the response.
    fn events(&self) -> Vec<Event>;

    /// Search for an event with given attribute key.
    fn event_attr_value(&self, event_type: &str, attr_key: &str) -> StdResult<String>;

    /// Get the data field of the response.
    fn data(&self) -> Option<Binary>;

    /// Helper to get the contract address of a instantiate response.
    fn instantiated_contract_address(&self) -> StdResult<Addr> {
        if let Ok(code_id) = self
            .event_attr_value(ADDRESS_INSTANTIATE_EVENT.0, ADDRESS_INSTANTIATE_EVENT.1)
            .map(Addr::unchecked)
        {
            Ok(code_id)
        } else {
            // for injective
            #[cfg(not(feature = "eth"))]
            panic!("Injective instantiate event parsing not supported without eth feature");
            #[cfg(feature = "eth")]
            return self
                .event_attr_value(
                    INJECTIVE_ADDRESS_INSTANTIATE_EVENT.0,
                    INJECTIVE_ADDRESS_INSTANTIATE_EVENT.1,
                )
                .map(|s| Addr::unchecked(unescape(&s).unwrap()));
        }
    }

    /// Shortcut to get the code id of a contract of an upload response.
    fn uploaded_code_id(&self) -> StdResult<u64> {
        if let Ok(code_id) = self
            .event_attr_value(CODE_ID_UPLOAD_EVENT.0, CODE_ID_UPLOAD_EVENT.1)
            .map(|s| s.parse().unwrap())
        {
            Ok(code_id)
        } else {
            // for injective
            #[cfg(not(feature = "eth"))]
            panic!("Injective upload event parsing not supported without eth feature");
            #[cfg(feature = "eth")]
            self.event_attr_value(
                INJECTIVE_CODE_ID_UPLOAD_EVENT.0,
                INJECTIVE_CODE_ID_UPLOAD_EVENT.1,
            )
            .map(|s| unescape(&s).unwrap().parse().unwrap())
        }
    }
}

#[cfg(test)]
mod index_response_test {
    use cosmwasm_std::{Addr, Event};

    use speculoos::prelude::*;

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
}
