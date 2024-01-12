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
    use cosmwasm_std::Addr;
    use cw_orch_core::{contract::interface_traits::*, environment::TxHandler};
    use cw_orch_daemon::Daemon;
    use cw_orch_traits::Stargate;
    use mock_contract::{InstantiateMsg, MigrateMsg, QueryMsg};
    use prost::Message;
    use prost::Name;
    use prost_types::Any;

    use speculoos::prelude::*;

    use crate::common::Id;

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

        let second_daemon = Daemon::builder()
            .chain(networks::LOCAL_JUNO)
            .handle(runtime.handle())
            .mnemonic(SECOND_MNEMONIC)
            .build()
            .unwrap();

        let sender = daemon.sender().to_string();
        let grantee = second_daemon.sender().to_string();

        // We start by granting authz to an account

        daemon.commit_any::<cosmos_sdk_proto::cosmos::MsgGrantResponse>(
            vec![Any {
                type_url: "cosmos.authz.v1beta1.MsgGrant".to_string(),
                value: MsgGrant {
                    granter: sender,
                    grantee: grantee.clone(),
                    grant: Some(cosmrs::proto::cosmos::authz::v1beta1::Grant {
                        authorization: Some(cosmrs::Any {
                            type_url: "cosmos.authz.v1beta1.GenericAuthorization".to_string(),
                            value: GenericAuthorization {
                                msg: MsgSend::full_name(),
                            }
                            .encode_to_vec(),
                        }),
                        expiration: None,
                    }),
                }
                .encode_to_vec(),
            }],
            None,
        )?;
        Ok(())
    }
}
