mod common;
#[cfg(feature = "node-tests")]
mod tests {
    /*
        Authz tests
    */
    use cosmrs::proto::cosmos::{
        authz::v1beta1::{
            GenericAuthorization, GrantAuthorization, MsgGrant, QueryGranteeGrantsResponse,
            QueryGranterGrantsResponse, QueryGrantsResponse,
        },
        bank::v1beta1::MsgSend,
    };
    use cosmwasm_std::coins;
    use cw_orch_core::environment::QuerierGetter;
    use cw_orch_core::environment::{BankQuerier, DefaultQueriers, QueryHandler, TxHandler};
    use cw_orch_daemon::{queriers::Authz, senders::CosmosOptions, Daemon};
    use cw_orch_networks::networks::LOCAL_JUNO;
    use cw_orch_traits::Stargate;
    use prost::Message;
    use prost::Name;
    use prost_types::Any;
    pub const SECOND_MNEMONIC: &str ="salute trigger antenna west ignore own dance bounce battle soul girl scan test enroll luggage sorry distance traffic brand keen rich syrup wood repair";

    #[test]
    #[serial_test::serial]
    fn authz() -> anyhow::Result<()> {
        super::common::enable_logger();
        use cw_orch_networks::networks;

        let daemon = Daemon::builder(networks::LOCAL_JUNO)
            .is_test(true)
            .build()
            .unwrap();

        let sender = daemon.sender_addr();

        let second_daemon: Daemon = daemon
            .rebuild()
            .build_sender(
                CosmosOptions::default()
                    .mnemonic(SECOND_MNEMONIC)
                    .authz_granter(&sender),
            )
            .unwrap();

        let grantee = second_daemon.sender_addr();

        let current_timestamp = daemon.block_info()?.time;

        let authorization = cosmrs::Any {
            type_url: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
            value: GenericAuthorization {
                msg: MsgSend::type_url(),
            }
            .encode_to_vec(),
        };
        let expiration = cosmrs::proto::Timestamp {
            seconds: (current_timestamp.seconds() + 3600) as i64,
            nanos: 0,
        };
        let grant = cosmrs::proto::cosmos::authz::v1beta1::Grant {
            authorization: Some(authorization.clone()),
            expiration: Some(expiration),
        };

        // We start by granting authz to an account
        daemon.commit_any(
            vec![Any {
                type_url: "/cosmos.authz.v1beta1.MsgGrant".to_string(),
                value: MsgGrant {
                    granter: sender.to_string(),
                    grantee: grantee.to_string(),
                    grant: Some(grant.clone()),
                }
                .encode_to_vec(),
            }],
            None,
        )?;

        // Check Queries of the authz
        let grant_authorization = GrantAuthorization {
            granter: sender.to_string(),
            grantee: grantee.to_string(),
            authorization: Some(authorization.clone()),
            expiration: Some(expiration),
        };

        // Grants
        let authz_querier: Authz = daemon.querier();
        let grants: QueryGrantsResponse = block_on(async {
            authz_querier
                ._grants(&sender, &grantee, MsgSend::type_url(), None)
                .await
        })?;
        assert_eq!(grants.grants, vec![grant]);

        // Grantee grants
        let grantee_grants: QueryGranteeGrantsResponse =
            block_on(async { authz_querier._grantee_grants(&grantee, None).await })?;
        assert_eq!(grantee_grants.grants, vec![grant_authorization.clone()]);

        // Granter grants
        let granter_grants: QueryGranterGrantsResponse =
            block_on(async { authz_querier._granter_grants(&sender, None).await })?;
        assert_eq!(granter_grants.grants, vec![grant_authorization]);

        // No grant gives out an error
        block_on(async {
            authz_querier
                ._grants(&grantee, &sender, MsgSend::type_url(), None)
                .await
        })
        .unwrap_err();

        // Check use of grants

        // The we send some funds to the account
        block_on(
            daemon
                .sender()
                .bank_send(&grantee, coins(100_000, LOCAL_JUNO.gas_denom)),
        )?;
        use async_std::task::block_on;
        // And send a large amount of tokens on their behalf
        block_on(
            second_daemon
                .sender()
                .bank_send(&grantee, coins(500_000, LOCAL_JUNO.gas_denom)),
        )?;

        // the balance of the grantee whould be 600_000 or close

        let grantee_balance = daemon
            .bank_querier()
            .balance(&grantee, Some(LOCAL_JUNO.gas_denom.to_string()))?;

        // One coin eaten by gas
        assert_eq!(grantee_balance.first().unwrap().amount.u128(), 600_000 - 1);

        Ok(())
    }
}
