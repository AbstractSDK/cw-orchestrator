mod common;

#[cfg(feature = "node-tests")]
mod sender {
    use crate::common;
    use cosmwasm_std::Uint128;
    use cw_orch::{
        daemon::error::DaemonError,
        prelude::{networks::LOCAL_JUNO, *},
    };
    use speculoos::{result::ResultAssertions, *};
    use tokio::runtime::Runtime;

    #[test]
    fn tx_not_found_after_x() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut bad_config = LOCAL_JUNO;
        bad_config.gas_denom = "baddenom";

        let daemon = SyncDaemon::builder()
            .chain(bad_config)
            .handle(rt.handle())
            .build()
            .unwrap();
        let cw20 = common::contract::Cw20::new(daemon);

        // attempt to execute with a wrong fee denom, hash won't be included in the block
        let res = cw20.upload();

        // we expect an error
        asserting!("the transaction is not found")
            .that(&res)
            .is_err()
            .matches(|e| e.to_string().contains("not found after"));
    }

    #[test]
    fn cosmwasm_exec_fails() {
        let runtime = Runtime::new().unwrap();
        let (sender, contract) = common::contract::start(&runtime);

        // upload contract
        let upload_res = contract.upload();
        asserting!("upload is succesful").that(&upload_res).is_ok();

        // init msg for contract
        let init_msg = common::contract::get_init_msg(&sender);
        contract
            .instantiate(&init_msg, Some(&sender), None)
            .unwrap();

        // burn more than we have amount
        let res = contract.execute(
            &cw20_base::msg::ExecuteMsg::Burn {
                amount: Uint128::from(1000000000000000000u128),
            },
            None,
        );

        // we expect an error
        asserting!("contract execution failed")
            .that(&res)
            .is_err()
            .matches(|e| match e {
                CwOrchError::DaemonError(DaemonError::Status(_)) => true,
                _ => false,
            });
    }
}
