use cosmwasm_std::BlockInfo;

use crate::environment::IndexResponse;
use std::fmt::Debug;

use super::Querier;

pub trait NodeQuerierGetter<E> {
    type Querier: NodeQuerier<Error = E>;
    fn node_querier(&self) -> Self::Querier;
}
pub trait NodeQuerier: Querier {
    type Response: IndexResponse + Debug + Send + Clone;

    /// Returns latests block information
    fn latest_block(&self) -> Result<BlockInfo, Self::Error>;

    /// Returns block information fetched by height
    fn block_by_height(&self, height: u64) -> Result<BlockInfo, Self::Error>;

    /// Returns current block height
    fn block_height(&self) -> Result<u64, Self::Error>;
    /// Returns the block timestamp (since unix epoch) in nanos
    fn block_time(&self) -> Result<u128, Self::Error>;

    /// Simulate TX
    fn simulate_tx(&self, tx_bytes: Vec<u8>) -> Result<u64, Self::Error>;

    /// Find TX by hash
    fn find_tx(&self, hash: String) -> Result<Self::Response, Self::Error>;
}
