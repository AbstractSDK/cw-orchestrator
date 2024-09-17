mod cw1_subkeys {
    use cw1_whitelist::msg::AdminListResponse;
    use cw_orch::{mock::Mock, prelude::*};
    use cw_plus_orch::cw1_subkeys::{
        Cw1SubKeys, ExecuteMsgInterfaceFns, InstantiateMsg, QueryMsgInterfaceFns,
    };

    #[test]
    fn check_interface() {
        let chain = Mock::new("sender");
        let contract = Cw1SubKeys::new("cw1", chain.clone());
        contract.upload().unwrap();
        contract
            .instantiate(
                &InstantiateMsg {
                    admins: vec![chain.sender_addr().to_string()],
                    mutable: true,
                },
                None,
                &[],
            )
            .unwrap();
        contract.execute_requests(vec![]).unwrap();

        let admins = contract.admin_list().unwrap();
        assert_eq!(
            admins,
            AdminListResponse {
                admins: vec![chain.sender_addr().to_string()],
                mutable: true
            }
        );
        contract.freeze().unwrap();
        let admins = contract.admin_list().unwrap();
        assert_eq!(
            admins,
            AdminListResponse {
                admins: vec![chain.sender_addr().to_string()],
                mutable: false
            }
        )
    }
}

mod cw1_whitelist {
    use cw1_whitelist::msg::AdminListResponse;
    use cw_orch::{mock::Mock, prelude::*};
    use cw_plus_orch::cw1_whitelist::{
        Cw1Whitelist, ExecuteMsgInterfaceFns, InstantiateMsg, QueryMsgInterfaceFns,
    };

    #[test]
    fn check_interface() {
        let chain = Mock::new("sender");
        let contract = Cw1Whitelist::new("cw1", chain.clone());
        contract.upload().unwrap();
        contract
            .instantiate(
                &InstantiateMsg {
                    admins: vec![chain.sender_addr().to_string()],
                    mutable: true,
                },
                None,
                &[],
            )
            .unwrap();
        contract.execute_requests(vec![]).unwrap();

        let admins = contract.admin_list().unwrap();
        assert_eq!(
            admins,
            AdminListResponse {
                admins: vec![chain.sender_addr().to_string()],
                mutable: true
            }
        );
        contract.freeze().unwrap();
        let admins = contract.admin_list().unwrap();
        assert_eq!(
            admins,
            AdminListResponse {
                admins: vec![chain.sender_addr().to_string()],
                mutable: false
            }
        )
    }
}

mod cw3_fixed_multisig {
    use cw3_fixed_multisig::msg::Voter;
    use cw_orch::{mock::Mock, prelude::*};
    use cw_plus_orch::cw3_fixed_multisig::{
        Cw3FixedMultisig, ExecuteMsgInterfaceFns, InstantiateMsg, QueryMsgInterfaceFns,
    };

    #[test]
    fn check_interface() {
        let chain = Mock::new("sender");
        let voter = chain.addr_make("voter");
        let contract = Cw3FixedMultisig::new("cw3", chain.clone());
        contract.upload().unwrap();
        contract
            .instantiate(
                &InstantiateMsg {
                    voters: vec![
                        Voter {
                            addr: voter.to_string(),
                            weight: 1,
                        },
                        Voter {
                            addr: chain.sender_addr().to_string(),
                            weight: 1,
                        },
                    ],
                    threshold: cw_utils::Threshold::AbsoluteCount { weight: 2 },
                    max_voting_period: cw_utils::Duration::Time(42424242),
                },
                None,
                &[],
            )
            .unwrap();
        contract
            .call_as(&voter)
            .propose("foobar", vec![], "title", None)
            .unwrap();
        let proposals = contract.list_proposals(None, None).unwrap();
        let proposal_id = proposals.proposals[0].id;
        contract.vote(proposal_id, cw3::Vote::Yes).unwrap();
        contract.execute_proposal(proposal_id).unwrap();

        let proposal = contract.proposal(proposal_id).unwrap();
        assert_eq!(proposal.status, cw3::Status::Executed);
    }
}

mod cw4_group_cw3_flex_multisig {
    use cw_orch::{mock::Mock, prelude::*};
    use cw_plus_orch::{
        cw3_flex_multisig::{
            Cw3FlexMultisig, ExecuteMsgInterfaceFns as _, InstantiateMsg as Cw3InstantiateMsg,
            QueryMsgInterfaceFns as _,
        },
        cw4_group::{
            Cw4Group, ExecuteMsgInterfaceFns, InstantiateMsg as Cw4InstantiateMsg,
            QueryMsgInterfaceFns as _,
        },
    };

    #[test]
    fn check_interface() {
        let chain = Mock::new("sender");
        let voter1 = chain.addr_make("voter1");
        let voter2 = chain.addr_make("voter2");
        let cw4 = Cw4Group::new("cw4", chain.clone());
        cw4.upload().unwrap();
        cw4.instantiate(
            &Cw4InstantiateMsg {
                admin: Some(chain.sender_addr().to_string()),
                members: vec![
                    cw4::Member {
                        addr: voter1.to_string(),
                        weight: 1,
                    },
                    cw4::Member {
                        addr: voter2.to_string(),
                        weight: 1,
                    },
                ],
            },
            None,
            &[],
        )
        .unwrap();
        chain.wait_blocks(10).unwrap();

        let hook_addr = chain.addr_make("hook");
        cw4.add_hook(hook_addr.to_string()).unwrap();
        let hooks = cw4.hooks().unwrap();
        assert_eq!(hooks.hooks, vec![hook_addr.to_string()]);
        cw4.remove_hook(hook_addr.to_string()).unwrap();
        let hooks = cw4.hooks().unwrap();
        assert!(hooks.hooks.is_empty());

        let cw3 = Cw3FlexMultisig::new("cw3", chain.clone());
        cw3.upload().unwrap();
        cw3.instantiate(
            &Cw3InstantiateMsg {
                group_addr: cw4.address().unwrap().to_string(),
                threshold: cw_utils::Threshold::AbsoluteCount { weight: 2 },
                max_voting_period: cw_utils::Duration::Time(1111111),
                executor: Some(cw3_flex_multisig::state::Executor::Only(
                    chain.sender_addr(),
                )),
                proposal_deposit: None,
            },
            None,
            &[],
        )
        .unwrap();
        cw3.call_as(&voter1)
            .propose("foobar", vec![], "title", None, &[])
            .unwrap();
        let proposals = cw3.list_proposals(None, None).unwrap();
        let proposal_id = proposals.proposals[0].id;
        cw3.call_as(&voter2)
            .vote(proposal_id, cw3::Vote::Yes)
            .unwrap();
        cw3.execute_proposal(proposal_id).unwrap();

        let proposal = cw3.proposal(proposal_id).unwrap();
        assert_eq!(proposal.status, cw3::Status::Executed);
    }
}

mod cw4_stake {
    use cosmwasm_std::{coins, Uint128};
    use cw_orch::{mock::Mock, prelude::*};
    use cw_plus_orch::cw4_stake::{
        Cw4Stake, ExecuteMsgInterfaceFns, InstantiateMsg, QueryMsgInterfaceFns,
    };

    #[test]
    fn check_interface() {
        let chain = Mock::new("sender");

        let cw4 = Cw4Stake::new("cw4", chain.clone());
        cw4.upload().unwrap();
        cw4.instantiate(
            &InstantiateMsg {
                denom: cw20::Denom::Native("abc".to_owned()),
                tokens_per_weight: Uint128::one(),
                min_bond: Uint128::one(),
                unbonding_period: cw_utils::Duration::Time(1231230000),
                admin: None,
            },
            None,
            &[],
        )
        .unwrap();

        let user1 = chain.addr_make("one");
        let user2 = chain.addr_make("two");

        chain.add_balance(&user1, coins(100, "abc")).unwrap();
        chain.add_balance(&user2, coins(300, "abc")).unwrap();
        cw4.call_as(&user1).bond(&coins(100, "abc")).unwrap();
        cw4.call_as(&user2).bond(&coins(300, "abc")).unwrap();

        let members = cw4.list_members(None, None).unwrap().members;
        assert!(members.contains(&cw4::Member {
            addr: user1.to_string(),
            weight: 100
        }));
        assert!(members.contains(&cw4::Member {
            addr: user2.to_string(),
            weight: 300
        }));
    }
}

mod cw20_base {
    use cosmwasm_std::Uint128;
    use cw20::MinterResponse;
    use cw_orch::{mock::Mock, prelude::*};
    use cw_plus_orch::cw20_base::{
        Cw20Base, ExecuteMsgInterfaceFns, InstantiateMsg, QueryMsgInterfaceFns,
    };

    #[test]
    fn check_interface() {
        let chain = Mock::new("sender");

        let cw20 = Cw20Base::new("cw20", chain.clone());
        cw20.upload().unwrap();
        cw20.instantiate(
            &InstantiateMsg {
                name: "foobar".to_owned(),
                symbol: "foo".to_owned(),
                decimals: 12,
                initial_balances: vec![],
                mint: Some(MinterResponse {
                    minter: chain.sender_addr().to_string(),
                    cap: None,
                }),
                marketing: None,
            },
            Some(&chain.sender_addr()),
            &[],
        )
        .unwrap();

        let user1 = chain.addr_make("one");
        let user2 = chain.addr_make("two");

        // Mint 100 for user1
        cw20.mint(100u128, user1.to_string()).unwrap();
        // User 1 shares 10 with user2
        cw20.call_as(&user1)
            .transfer(10u128, user2.to_string())
            .unwrap();

        // Check new balance of user1
        let balance = cw20.balance(user1.to_string()).unwrap().balance;
        assert_eq!(balance, Uint128::new(90));

        // Check that user2 registered
        let accounts = cw20.all_accounts(None, None).unwrap().accounts;
        assert!(accounts.contains(&user2.to_string()));

        // Can "migrate" with empty
        cw20.migrate(&Empty {}, cw20.code_id().unwrap()).unwrap();
    }
}

mod cw20_ics {
    use cosmwasm_std::{coins, to_json_binary, Uint128};
    use cw20::MinterResponse;
    use cw20_base::msg::InstantiateMsg;
    use cw20_ics20::{
        ibc::{ICS20_ORDERING, ICS20_VERSION},
        msg::AllowedInfo,
    };
    use cw_orch::prelude::*;
    use cw_orch_interchain::{env::contract_port, prelude::*};
    use cw_plus_orch::{
        cw20_base::{Cw20Base, ExecuteMsgInterfaceFns as _},
        cw20_ics20::{
            AllowMsg, Cw20Ics20, ExecuteMsgInterfaceFns, InitMsg, QueryMsgInterfaceFns, TransferMsg,
        },
    };

    #[test]
    fn check_interface() {
        let interchain =
            MockBech32InterchainEnv::new(vec![("juno-1", "sender"), ("stargaze-1", "sender")]);

        let juno = interchain.get_chain("juno-1").unwrap();
        let stargaze = interchain.get_chain("stargaze-1").unwrap();

        let gov_juno = juno.addr_make("gov");

        let cw20 = Cw20Base::new("cw20", juno.clone());
        cw20.upload().unwrap();
        cw20.instantiate(
            &InstantiateMsg {
                name: "foobar".to_owned(),
                symbol: "foo".to_owned(),
                decimals: 12,
                initial_balances: vec![],
                mint: Some(MinterResponse {
                    minter: juno.sender_addr().to_string(),
                    cap: None,
                }),
                marketing: None,
            },
            None,
            &[],
        )
        .unwrap();
        let cw20_ics20 = Cw20Ics20::new("cw20_ics20", juno.clone());
        cw20_ics20.upload().unwrap();
        cw20_ics20
            .instantiate(
                &InitMsg {
                    default_timeout: 3600,
                    gov_contract: gov_juno.to_string(),
                    allowlist: vec![AllowMsg {
                        contract: cw20.addr_str().unwrap(),
                        gas_limit: None,
                    }],
                    default_gas_limit: None,
                },
                None,
                &[],
            )
            .unwrap();
        let allow_list = cw20_ics20.list_allowed(None, None).unwrap().allow;
        assert_eq!(
            allow_list,
            vec![AllowedInfo {
                contract: cw20.addr_str().unwrap(),
                gas_limit: None
            }]
        );

        let channel = interchain
            .create_channel(
                "juno-1",
                "stargaze-1",
                &contract_port(&cw20_ics20),
                &PortId::transfer(),
                ICS20_VERSION,
                Some(ICS20_ORDERING),
            )
            .unwrap();
        let channel = channel
            .interchain_channel
            .get_ordered_ports_from("juno-1")
            .unwrap();
        let channels = cw20_ics20.list_channels().unwrap().channels;
        let expected_channel_id = channel.0.channel.unwrap().to_string();
        assert_eq!(channels[0].id, expected_channel_id);

        let user_juno = juno.addr_make("juno");
        let user_stargaze = stargaze.addr_make("stargaze");

        let transfer_msg = TransferMsg {
            channel: expected_channel_id,
            remote_address: user_stargaze.to_string(),
            timeout: None,
            memo: None,
        };
        // Send 100 cw20 coins to stargaze
        cw20.mint(100_u128, user_juno.to_string()).unwrap();
        let response = cw20
            .call_as(&user_juno)
            .send(
                100_u128,
                cw20_ics20.addr_str().unwrap(),
                to_json_binary(&transfer_msg).unwrap(),
            )
            .unwrap();
        interchain
            .await_and_check_packets("juno-1", response)
            .unwrap();
        // Send 200 native coins to stargaze
        juno.add_balance(&user_juno, coins(200, "denom")).unwrap();
        let response = cw20_ics20
            .call_as(&user_juno)
            .transfer(transfer_msg, &coins(200, "denom"))
            .unwrap();
        interchain
            .await_and_check_packets("juno-1", response)
            .unwrap();

        let balance = stargaze.balance(&user_stargaze, None).unwrap();
        assert_eq!(balance[0].amount, Uint128::new(100));
        assert_eq!(balance[1].amount, Uint128::new(200));
    }
}
