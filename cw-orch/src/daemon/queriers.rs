//! # DaemonQuerier
//!
//! Daemon queriers are gRPC query clients for the CosmosSDK modules. They can be used to query the different modules (Bank, Ibc, Authz, ...).
//!
//! ## Usage
//!
//! You will need to acquire a [gRPC channel](Channel) to a running CosmosSDK node to be able to use the queriers.  
//! Here is an example of how to acquire one using the Daemon builder.
//!
//! ```no_run
//! // include tokio runtime
//! use tokio::runtime::Runtime;
//!
//! // require the querier you want to use, in this case Node
//! use cw_orch::{queriers::Node, Daemon, networks, queriers::DaemonQuerier};
//!
//! // prepare a runtime
//! let runtime = Runtime::new().unwrap();
//!
//! // call the builder and configure it as you need
//! let daemon = Daemon::builder()
//!     .chain(networks::LOCAL_JUNO)
//!     .handle(runtime.handle())
//!     .build()
//!     .unwrap();
//!
//! // now you can use the Node querier:
//! let node = Node::new(daemon.channel());
//! let node_info = node.info();
//! ```
//!
//! ### Node querier
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::Node, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let node = Node::new(daemon.channel());
//! let block_height = node.block_height();
//! ```
//!
//! ### Bank querier
//!
//! #### coin_balance
//!
//! Fetch the bank balance of a given address If denom is None, returns all balances
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::Bank, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let bank = Bank::new(daemon.channel());
//! let holder_address = "...";
//! let denom = "ujunox".to_string();
//! let balance = bank.coin_balance(holder_address, Some(denom));
//! ```
//!
//! #### total_supply
//!
//! Fetch total supply in the bank module
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::Bank, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let bank = Bank::new(daemon.channel());
//! let supply = bank.total_supply();
//! ```
//!
//! ### Gov querier
//!
//! #### proposals
//!
//! Fetch all proposals based on given status.
//!
//! Unspecified = 0, DepositPeriod = 1, VotingPeriod = 2, Passed = 3, Rejected = 4, Failed = 5,
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
//! # use cw_orch::{queriers::Gov, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let gov = Gov::new(daemon.channel());
//! let proposal_status = 0i32;
//! let voter = "...";
//! let depositor = "...";
//! let pagination = PageRequest { key: vec![], offset: 0u64, limit: 30u64, count_total: true, reverse: false };
//! let props = gov.proposals(proposal_status, voter, depositor, Some(pagination));
//! ```
//!
//! #### vote
//!
//! Fetch voter information for given proposal
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::Gov, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let gov = Gov::new(daemon.channel());
//! let proposal_id = 100u64;
//! let voter = "...";
//! let vote_info = gov.vote(proposal_id, voter);
//! ```
//! #### votes
//!
//! Fetch all votes information for given proposal
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
//! # use cw_orch::{queriers::Gov, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let gov = Gov::new(daemon.channel());
//! let proposal_id = 100u64;
//! let pagination = PageRequest { key: vec![], offset: 0u64, limit: 30u64, count_total: true, reverse: false };
//! let props = gov.votes(proposal_id, Some(pagination));
//! ```
//!
//! ### Staking querier
//!
//! #### validators
//!
//! Fetch list of validators under a given status:
//! BOND_STATUS_BONDED, BOND_STATUS_UNBONDING, BOND_STATUS_UNBONDED, BOND_STATUS_UNSPECIFIED
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::Staking, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let staking = Staking::new(daemon.channel());
//! let list = staking.validators("BOND_STATUS_BONDED");
//! ```
//!
//! #### delegation
//!
//! Fetch staked balance for given delegator to given validator
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::Staking, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let staking = Staking::new(daemon.channel());
//! let validator_addr = "...";
//! let delegator_addr = "...";
//! let staked = staking.delegation(validator_addr, delegator_addr);
//! ```
//!
//! #### unbonding_delegation
//!
//! Fetch all unbonding delegations of a given delegator address.
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::Staking, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let staking = Staking::new(daemon.channel());
//! let validator_addr = "...";
//! let delegator_addr = "...";
//! let unbonding = staking.unbonding_delegation(validator_addr, delegator_addr);
//! ```
//!
//! #### redelegations
//!
//! Fetch redelegations of given address.
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
//! # use cw_orch::{queriers::Staking, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let staking = Staking::new(daemon.channel());
//! let delegator_addr = "...";
//! let src_validator_addr = "...";
//! let dst_validator_addr = "...";
//! let pagination = PageRequest { key: vec![], offset: 0u64, limit: 30u64, count_total: true, reverse: false };
//! let redelegation = staking.redelegations(delegator_addr, src_validator_addr, dst_validator_addr, Some(pagination));
//! ```
//!
//! ### Feegrant
//!
//! #### allowance
//!
//! Fetch fee granted to a grantee
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::Feegrant, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let feegrant = Feegrant::new(daemon.channel());
//! let granter = "...";
//! let grantee = "...";
//! let allowance = feegrant.allowance(granter, grantee);
//! ```
//!
//! #### allowances
//!
//! Fetch all grants to a grantee
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
//! # use cw_orch::{queriers::Feegrant, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let feegrant = Feegrant::new(daemon.channel());
//! let grantee = "...";
//! let pagination = PageRequest { key: vec![], offset: 0u64, limit: 30u64, count_total: true, reverse: false };
//! let allowances = feegrant.allowances(grantee, Some(pagination));
//! ```
//!
//! ### CosmWasm
//!
//! #### contract_info
//!
//! Fetch contract information
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::CosmWasm, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let cw = CosmWasm::new(daemon.channel());
//! let contract_address = "...";
//! let info = cw.contract_info(contract_address);
//! ```
//!
//! #### contract_history
//!
//! Fetch contract history
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
//! # use cw_orch::{queriers::CosmWasm, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let cw = CosmWasm::new(daemon.channel());
//! let contract_address = "...";
//! let pagination = PageRequest { key: vec![], offset: 0u64, limit: 30u64, count_total: true, reverse: false };
//! let history = cw.contract_history(contract_address, Some(pagination));
//! ```
//!
//! ### IBC
//!
//! #### clients
//!
//! Fetch known clients
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::Ibc, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let ibc = Ibc::new(daemon.channel());
//! let clients = ibc.clients();
//! ```
//!
//! #### client_state
//!
//! Fetch the state of a specific IBC client
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use cw_orch::{queriers::Ibc, Daemon, networks, queriers::DaemonQuerier};
//! # let runtime = Runtime::new().unwrap();
//! # let daemon = Daemon::builder()
//! #    .chain(networks::LOCAL_JUNO)
//! #    .handle(runtime.handle())
//! #    .build()
//! #    .unwrap();
//!
//! let ibc = Ibc::new(daemon.channel());
//! let client_id = "...";
//! let state = ibc.client_state(client_id);
//! ```

#[macro_export]
macro_rules! cosmos_query {
    ($self:ident, $module:ident, $func_name:ident, $request_type:ident { $($field:ident : $value:expr),* $(,)?  }) => {
        {
        use $crate::daemon::cosmos_modules::$module::{
            query_client::QueryClient, $request_type,
        };
        let mut client = QueryClient::new($self.channel.clone());
        #[allow(clippy::redundant_field_names)]
        let request = $request_type { $($field : $value),* };
        let response = client.$func_name(request.clone()).await?.into_inner();
        ::log::trace!(
            "cosmos_query: {:?} resulted in: {:?}",
            request,
            response
        );
        response
    }
};
}

mod bank;
mod cosmwasm;
mod feegrant;
mod gov;
mod ibc;
mod node;
mod staking;

pub use bank::Bank;
pub use cosmwasm::CosmWasm;
pub use feegrant::Feegrant;
pub use gov::Gov;
pub use ibc::Ibc;
pub use node::Node;
pub use staking::Staking;

use tonic::transport::Channel;

/// Constructor for a querier over a given channel
pub trait DaemonQuerier {
    /// Construct an new querier over a given channel
    fn new(channel: Channel) -> Self;
}
