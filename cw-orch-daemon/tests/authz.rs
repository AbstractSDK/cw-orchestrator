mod common;
#[cfg(feature = "node-tests")]
mod tests {
    /*
        DaemonAsync contract general tests
    */

    use cosmrs::proto::cosmos::{
        authz::v1beta1::{GenericAuthorization, MsgGrant, MsgGrantResponse},
        bank::v1beta1::MsgSend,
    };
    use cosmwasm_std::coins;
    use cw_orch_core::environment::{BankQuerier, TxHandler};
    use cw_orch_daemon::Daemon;
    use cw_orch_networks::networks::LOCAL_JUNO;
    use cw_orch_traits::Stargate;
    use prost::Message;
    use prost::Name;
    use prost_types::{Any, Timestamp};

    pub const SECOND_MNEMONIC: &str ="salute trigger antenna west ignore own dance bounce battle soul girl scan test enroll luggage sorry distance traffic brand keen rich syrup wood repair";

    #[test]
    #[serial_test::serial]
    fn authz() -> anyhow::Result<()> {
        use cw_orch_networks::networks;

        let runtime = tokio::runtime::Runtime::new().unwrap();

        let daemon = Daemon::builder()
            .chain(networks::LOCAL_JUNO)
            .handle(runtime.handle())
            .build()
            .unwrap();

        let sender = daemon.sender().to_string();

        let second_daemon = Daemon::builder()
            .chain(networks::LOCAL_JUNO)
            .handle(runtime.handle())
            .with_authz(sender.clone())
            .mnemonic(SECOND_MNEMONIC)
            .build()
            .unwrap();

        let grantee = second_daemon.sender().to_string();

        let current_timestamp = daemon.block_info()?.time;

        // We start by granting authz to an account
        daemon.commit_any::<MsgGrantResponse>(
            vec![Any {
                type_url: "/cosmos.authz.v1beta1.MsgGrant".to_string(),
                value: MsgGrant {
                    granter: sender.clone(),
                    grantee: grantee.clone(),
                    grant: Some(cosmrs::proto::cosmos::authz::v1beta1::Grant {
                        authorization: Some(cosmrs::Any {
                            type_url: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
                            value: GenericAuthorization {
                                msg: MsgSend::type_url(),
                            }
                            .encode_to_vec(),
                        }),
                        expiration: Some(Timestamp {
                            seconds: (current_timestamp.seconds() + 3600) as i64,
                            nanos: 0,
                        }),
                    }),
                }
                .encode_to_vec(),
            }],
            None,
        )?;

        // The we send some funds to the account
        runtime.block_on(
            daemon
                .daemon
                .sender
                .bank_send(&grantee, coins(1_000_000, LOCAL_JUNO.gas_denom)),
        )?;

        // And send a large amount of tokens on their behalf
        runtime.block_on(
            second_daemon
                .daemon
                .sender
                .bank_send(&grantee, coins(5_000_000, LOCAL_JUNO.gas_denom)),
        )?;

        // the balance of the grantee whould be 6_000_000 or close

        let grantee_balance =
            daemon.balance(grantee.clone(), Some(LOCAL_JUNO.gas_denom.to_string()))?;

        assert_eq!(grantee_balance.first().unwrap().amount.u128(), 6_000_000);

        Ok(())
    }
}