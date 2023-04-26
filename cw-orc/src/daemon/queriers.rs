//! # DaemonQuerier
//!
//! ## Usage examples
//!
//! Note: all following examples need the following code:
//!
//! ```rust
//! use std::sync::Arc;
//! use tokio::runtime::Runtime;
//! let rt = Arc::new(Runtime::new().unwrap());
//! let channel = rt.block_on(build_channel()).unwrap();
//! ```
//!
//! ### Node querier
//!
//! ```rust
//! let node = Node::new(channel.clone());
//! let block_height = node.block_height();
//! ```
//!
//! ### Bank querier
//!
//! #### coin_balance
//!
//! Fetch the bank balance of a given address If denom is None, returns all balances
//!
//! ```rust
//! let bank = Bank::new(channel.clone());
//! let holder_address = "...";
//! let denom = "ujunox";
//! let balance = bank.coin_balance(holder_address, denom);
//! ```
//!
//! #### total_supply
//!
//! Fetch total supply in the bank module
//!
//! ```rust
//! let bank = Bank::new(channel.clone());
//! let pagination = PageRequest { offset: 0u64, limit: 30u64 };
//! let supply = bank.total_supply(pagination);
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
//! ```rust
//! let gov = Gov::new(channel.clone());
//! let proposal_status = 0i32;
//! let voter = "...";
//! let depositor = "...";
//! let pagination = PageRequest { offset: 0u64, limit: 30u64 };
//! let props = gov.proposals(proposal_status, voter, depositor, pagination);
//! ```
//! #### Vote
//!
//! Fetch voter information for given proposal
//!
//! ```rust
//! let gov = Gov::new(channel.clone());
//! let proposal_id = 100u64;
//! let voter = "...";
//! let props = gov.vote(proposal_id, voter);
//! ```
//!
//! ### Staking querier
//!
//! #### delegation
//!
//! Fetch staked balance for given delegator to given validator
//!
//! ```rust
//! let staking = Staking::new(channel.clone());
//! let validator_addr = "...";
//! let delegator_addr = "...";
//! let staked = staking.delegation(validator_addr, delegator_addr);
//! ```
//!
//! #### unbonding_delegation
//!
//! Fetch all unbonding delegations of a given delegator address.
//!
//! ```rust
//! let staking = Staking::new(channel.clone());
//! let validator_addr = "...";
//! let delegator_addr = "...";
//! let unbonding = staking.unbonding_delegation(validator_addr, delegator_addr);
//! ```
//!
//! #### redelegations
//!
//! Fetch redelegations of given address.
//!
//! ```rust
//! let staking = Staking::new(channel.clone());
//! let delegator_addr = "...";
//! let src_validator_addr = "...";
//! let dst_validator_addr = "...";
//! let pagination = PageRequest { offset: 0u64, limit: 30u64 };
//! let redelegation = staking.redelegations(delegator_addr, src_validator_addr, dst_validator_addr);
//! ```
//!
//! ### Feegrant
//!
//! #### allowance
//!
//! Fetch fee granted to a grantee
//!
//! ```rust
//! let feegrant = Feegrant::new(channel.clone());
//! let granter = "...";
//! let grantee = "...";
//! let allowance = feegrant.allowance(granter, grantee);
//! ```
//!
//! #### allowances
//!
//! Fetch all grants to a grantee
//!
//! ```rust
//! let feegrant = Feegrant::new(channel.clone());
//! let grantee = "...";
//! let pagination = PageRequest { offset: 0u64, limit: 30u64 };
//! let allowances = feegrant.allowances(granter, pagination);
//! ```
//!
//! ### CosmWasm
//!
//! #### contract_info
//!
//! Fetch contract information
//!
//! ```rust
//! let cw = CosmWasm::new(channel.clone());
//! let contract_address = "...";
//! let info = cw.contract_info(contract_address);
//! ```
//!
//! #### contract_history
//!
//! Fetch contract history
//!
//! ```rust
//! let cw = CosmWasm::new(channel.clone());
//! let contract_address = "...";
//! let pagination = PageRequest { offset: 0u64, limit: 30u64 };
//! let history = cw.contract_history(contract_address, pagination);
//! ```
//!
//! ### IBC
//!
//! #### clients
//!
//! Fetch known clients
//!
//! ```rust
//! let ibc = Ibc::new(channel.clone());
//! let clients = ibc.clients();
//! ```
//!
//! #### client_state
//!
//! Fetch the state of a specific IBC client
//!
//! ```rust
//! let ibc = Ibc::new(channel.clone());
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
