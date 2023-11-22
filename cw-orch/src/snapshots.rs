//! Defined the snapshot testing macro on the Mock environment
//! This is included here and not in the mock package because it needs to import traits to work

/// This functions allows for making sure the tests are stabilised and changes made to contracts don't have an impact on the internal storage
/// This should ONLY be used when cw_orch is in scope
/// Usage:
/// ```rust,ignore
/// mock.take_storage_snapshot("mock_doc");
/// ```
/// The name you input to the function should be different from all other snapshots in your repository
/// Find more details on how snapshot testing works on the official quick-start guide: https://insta.rs/docs/quickstart/
/// This function will panic if the snapshot is different from the reference snapshot
#[macro_export]
macro_rules! take_storage_snapshot {
    ($chain: ident, $name: literal) => {
        // We register and test a snapshot for all contracts storage
        use ::cw_orch::environment::{ChainState as _, StateInterface as _};
        let all_contract_addresses = $chain.state().get_all_addresses()?;
        for (_id, contract_addr) in all_contract_addresses {
            ::cw_orch::insta::assert_yaml_snapshot!(
                ::cw_orch::sanitize_filename::sanitize(format!("{}-{}", $name, contract_addr.to_string())),
                $chain.app.borrow().dump_wasm_raw(&contract_addr)
            );
        }
    };
}

#[cfg(test)]
pub mod tests {
    use cosmwasm_std::Addr;
    use cw_orch::prelude::{CwOrchInstantiate, CwOrchUpload, Mock};

    #[test]
    fn contract_snapshots() -> anyhow::Result<()> {
        use counter_contract::CounterExecuteMsgFns;
        let sender = Addr::unchecked("sender");
        let chain = Mock::new(&sender);

        let contract =
            counter_contract::CounterContract::new("test:counter_contract", chain.clone());
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
}
