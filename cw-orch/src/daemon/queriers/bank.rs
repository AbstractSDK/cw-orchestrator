use crate::{daemon::cosmos_modules, DaemonError};
use cosmrs::proto::cosmos::base::v1beta1::Coin;
use tonic::transport::Channel;

use super::DaemonQuerier;

/// Queries the node for information
pub struct Bank {
    channel: Channel,
}

impl DaemonQuerier for Bank {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Bank {
    /// Query the bank balance of a given address
    /// If denom is None, returns all balances
    pub async fn coin_balance(
        &self,
        address: impl Into<String>,
        denom: Option<impl Into<String>>,
    ) -> Result<Vec<Coin>, DaemonError> {
        use cosmos_modules::bank::query_client::QueryClient;
        match denom {
            Some(denom) => {
                let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
                let request = cosmos_modules::bank::QueryBalanceRequest {
                    address: address.into(),
                    denom: denom.into(),
                };
                let resp = client.balance(request).await?.into_inner();
                let coin = resp.balance.unwrap();
                Ok(vec![coin])
            }
            None => {
                let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
                let request = cosmos_modules::bank::QueryAllBalancesRequest {
                    address: address.into(),
                    ..Default::default()
                };
                let resp = client.all_balances(request).await?.into_inner();
                let coins = resp.balances;
                Ok(coins.into_iter().collect())
            }
        }
    }
}
