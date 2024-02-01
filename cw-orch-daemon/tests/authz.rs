mod common;
#[cfg(feature = "node-tests")]
mod tests {
    /*
        Authz tests
    */

    use cosmrs::proto::cosmos::{
        authz::v1beta1::{
            GenericAuthorization, GrantAuthorization, MsgGrant, MsgGrantResponse,
            QueryGranteeGrantsResponse, QueryGranterGrantsResponse, QueryGrantsResponse,
        },
        bank::v1beta1::MsgSend,
    };
    use cosmwasm_std::coins;
    use cw_orch_core::environment::{BankQuerier, DefaultQueriers, QueryHandler, TxHandler};
    use cw_orch_daemon::{queriers::Authz, Daemon};
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
            .authz_granter(sender.clone())
            .mnemonic(SECOND_MNEMONIC)
            .build()
            .unwrap();

        let grantee = second_daemon.sender().to_string();

        let current_timestamp = daemon.block_info()?.time;

        let authorization = cosmrs::Any {
            type_url: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
            value: GenericAuthorization {
                msg: MsgSend::type_url(),
            }
            .encode_to_vec(),
        };
        let expiration = Timestamp {
            seconds: (current_timestamp.seconds() + 3600) as i64,
            nanos: 0,
        };
        let grant = cosmrs::proto::cosmos::authz::v1beta1::Grant {
            authorization: Some(authorization.clone()),
            expiration: Some(expiration.clone()),
        };
        // We start by granting authz to an account
        daemon.commit_any::<MsgGrantResponse>(
            vec![Any {
                type_url: "/cosmos.authz.v1beta1.MsgGrant".to_string(),
                value: MsgGrant {
                    granter: sender.clone(),
                    grantee: grantee.clone(),
                    grant: Some(grant.clone()),
                }
                .encode_to_vec(),
            }],
            None,
        )?;

        // Check Queries of the authz
        let grant_authorization = GrantAuthorization {
            granter: sender.clone(),
            grantee: grantee.clone(),
            authorization: Some(authorization.clone()),
            expiration: Some(expiration.clone()),
        };

        // Grants
        let authz_querier: Authz = daemon.query_client();
        let grants: QueryGrantsResponse = runtime.handle().block_on(async {
            authz_querier
                .grants(sender.clone(), grantee.clone(), MsgSend::type_url(), None)
                .await
        })?;
        assert_eq!(grants.grants, vec![grant]);

        // Grantee grants
        let grantee_grants: QueryGranteeGrantsResponse = runtime
            .handle()
            .block_on(async { authz_querier.grantee_grants(grantee.clone(), None).await })?;
        assert_eq!(grantee_grants.grants, vec![grant_authorization.clone()]);

        // Granter grants
        let granter_grants: QueryGranterGrantsResponse = runtime
            .handle()
            .block_on(async { authz_querier.granter_grants(sender.clone(), None).await })?;
        assert_eq!(granter_grants.grants, vec![grant_authorization]);

        // No grant gives out an error
        runtime
            .handle()
            .block_on(async {
                authz_querier
                    .grants(grantee.clone(), sender.clone(), MsgSend::type_url(), None)
                    .await
            })
            .unwrap_err();

        // Check use of grants

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

        let grantee_balance = daemon
            .bank_querier()
            .balance(grantee.clone(), Some(LOCAL_JUNO.gas_denom.to_string()))?;

        assert_eq!(grantee_balance.first().unwrap().amount.u128(), 6_000_000);

        Ok(())
    }
}
