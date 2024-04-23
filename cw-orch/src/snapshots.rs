//! Defined the snapshot testing macro on the Mock environment
//! This is included here and not in the mock package because it needs to import traits to work

/// Function helper used to parse storage into readable strings
pub fn parse_storage(storage: &[(Vec<u8>, Vec<u8>)]) -> Vec<(String, String)> {
    storage
        .iter()
        .map(|(key, value)| {
            (
                String::from_utf8_lossy(key).to_string(),
                String::from_utf8_lossy(value).to_string(),
            )
        })
        .collect::<Vec<_>>()
}
/// This functions allows for making sure the tests are stabilised and changes made to contracts don't have an impact on the internal storage
/// This should ONLY be used when cw_orch is in scope
/// Usage:
/// ```rust,ignore
/// take_storage_snapshot!(chain, "mock_doc");
/// ```
/// The name you input to the function should be different from all other snapshots in your repository
/// Find more details on how snapshot testing works on the official quick-start guide: https://insta.rs/docs/quickstart/
/// This function will panic if the snapshot is different from the reference snapshot

#[macro_export]
macro_rules! take_storage_snapshot {
    ($chain: ident, $name: literal) => {
        // We register and test a snapshot for all contracts storage
        use ::cw_orch::environment::{ChainState as _, StateInterface as _};
        let all_contracts = $chain.state().get_all_addresses()?;
        let all_storage: ::std::collections::BTreeMap<_, _> = all_contracts
            .iter()
            .map(|(id, contract_addr)| {
                (
                    id,
                    ::cw_orch::snapshots::parse_storage(
                        &$chain.app.borrow().dump_wasm_raw(&contract_addr),
                    ),
                )
            })
            .collect();

        ::cw_orch::insta::assert_yaml_snapshot!(
            ::cw_orch::sanitize_filename::sanitize(format!("{}", $name)),
            all_storage
        )
    };
}

#[cfg(test)]
pub mod tests {
    use crate::mock::cw_multi_test::ContractWrapper;
    use cosmwasm_std::Empty;
    use cw_orch::prelude::{CwOrchInstantiate, CwOrchUpload, Mock};
    use cw_orch_core::{
        contract::{interface_traits::Uploadable, WasmPath},
        environment::ChainInfo,
    };

    #[test]
    fn contract_snapshots() -> anyhow::Result<()> {
        use counter_contract::CounterExecuteMsgFns;
        let chain = Mock::new("sender");

        let contract = counter_contract::CounterContract::new(chain.clone());
        contract.upload()?;
        contract.instantiate(
            &counter_contract::msg::InstantiateMsg { count: 0 },
            None,
            None,
        )?;

        contract.increment()?;

        take_storage_snapshot!(chain, "snapshot_test");

        Ok(())
    }

    #[cw_orch::interface(
        counter_contract::msg::InstantiateMsg,
        counter_contract::msg::ExecuteMsg,
        counter_contract::msg::QueryMsg,
        counter_contract::msg::MigrateMsg
    )]
    pub struct CounterContractWithId;

    impl<Chain> Uploadable for CounterContractWithId<Chain> {
        /// Return the path to the wasm file corresponding to the contract
        fn wasm(_chain: &ChainInfo) -> WasmPath {
            unimplemented!()
        }
        /// Returns a CosmWasm contract wrapper
        fn wrapper() -> Box<dyn cw_orch::prelude::MockContract<Empty>> {
            Box::new(
                ContractWrapper::new_with_empty(
                    counter_contract::contract::execute,
                    counter_contract::contract::instantiate,
                    counter_contract::contract::query,
                )
                .with_migrate(counter_contract::contract::migrate),
            )
        }
    }

    #[test]
    fn multiple_contract_snapshot() -> anyhow::Result<()> {
        use counter_contract::CounterExecuteMsgFns;
        let chain = Mock::new("sender");

        let contract = counter_contract::CounterContract::new(chain.clone());
        contract.upload()?;
        contract.instantiate(
            &counter_contract::msg::InstantiateMsg { count: 0 },
            None,
            None,
        )?;
        contract.increment()?;

        let contract = CounterContractWithId::new("second-counter-contract", chain.clone());
        contract.upload()?;
        contract.instantiate(
            &counter_contract::msg::InstantiateMsg { count: 0 },
            None,
            None,
        )?;
        contract.increment()?;
        contract.increment()?;
        contract.increment()?;
        contract.increment()?;

        let contract = CounterContractWithId::new("third-counter-contract", chain.clone());
        contract.upload()?;
        contract.instantiate(
            &counter_contract::msg::InstantiateMsg { count: 0 },
            None,
            None,
        )?;
        contract.increment()?;
        contract.increment()?;
        contract.increment()?;

        take_storage_snapshot!(chain, "multiple_contracts_snapshot_test");

        Ok(())
    }
}
