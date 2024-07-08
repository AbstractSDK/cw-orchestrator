use std::{str::FromStr, sync::Arc};

use cosmrs::AccountId;
use cw_orch_core::environment::ChainInfoOwned;

use crate::{DaemonError, Wallet};

use super::{builder::SenderBuilder, CosmosSender};

/// Options for how txs should be constructed for this sender.
#[derive(Default, Clone)]
#[non_exhaustive]
pub struct CosmosOptions {
    pub authz_granter: Option<String>,
    pub fee_granter: Option<String>,
    pub hd_index: Option<u32>,
    /// Used to derive the private key
    pub(crate) key: CosmosWalletKey,
}

#[derive(Default, Clone)]
pub enum CosmosWalletKey {
    Mnemonic(String),
    RawKey(Vec<u8>),
    #[default]
    Env,
}

impl CosmosOptions {
    pub fn check(&self) -> Result<(), DaemonError> {
        if let Some(addr) = &self.authz_granter {
            AccountId::from_str(addr)?;
        }

        if let Some(addr) = &self.fee_granter {
            AccountId::from_str(addr)?;
        }

        Ok(())
    }

    pub fn authz_granter(mut self, granter: impl ToString) -> Self {
        self.authz_granter = Some(granter.to_string());
        self
    }

    pub fn fee_granter(mut self, granter: impl ToString) -> Self {
        self.fee_granter = Some(granter.to_string());
        self
    }

    pub fn hd_index(mut self, index: u32) -> Self {
        self.hd_index = Some(index);
        self
    }

    pub fn mnemonic(mut self, mnemonic: impl ToString) -> Self {
        self.key = CosmosWalletKey::Mnemonic(mnemonic.to_string());
        self
    }

    pub fn set_authz_granter(&mut self, granter: impl ToString) {
        self.authz_granter = Some(granter.to_string());
    }

    pub fn set_fee_granter(&mut self, granter: impl ToString) {
        self.fee_granter = Some(granter.to_string());
    }

    pub fn set_hd_index(&mut self, index: u32) {
        self.hd_index = Some(index);
    }

    pub fn set_mnemonic(&mut self, mnemonic: impl ToString) {
        self.key = CosmosWalletKey::Mnemonic(mnemonic.to_string());
    }
}

impl SenderBuilder for CosmosOptions {
    type Error = DaemonError;
    type Sender = Wallet;

    async fn build(&self, chain_info: &Arc<ChainInfoOwned>) -> Result<Self::Sender, Self::Error> {
        CosmosSender::new(chain_info, self.clone()).await
    }
}
