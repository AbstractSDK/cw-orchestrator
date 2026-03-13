#![allow(missing_docs)]

use crate::DaemonError;
use cosmrs::tx::Raw;
use cosmrs::tx::SignDoc;
use prost::Name;

#[cfg(feature = "eth")]
use crate::keys::private::PrivateKey;

#[cfg(feature = "eth")]
use ::{cosmrs::proto, ethers_core::utils::keccak256};

pub const ETHEREUM_COIN_TYPE: u32 = 60;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectiveEthAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<super::super::cosmos_modules::auth::BaseAccount>,
    #[prost(bytes, tag = "2")]
    pub code_hash: Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectivePubKey {
    #[prost(bytes, tag = 1)]
    pub key: Vec<u8>,
}

impl Name for InjectivePubKey {
    const NAME: &'static str = "PubKey";
    const PACKAGE: &'static str = "injective.crypto.v1beta1.ethsecp256k1";

    /// Workaround until tokio-rs/prost#923 is released
    fn full_name() -> String {
        format!("{}.{}", Self::PACKAGE, Self::NAME)
    }
}

pub trait InjectiveSigner {
    fn sign_injective(&self, sign_doc: SignDoc) -> Result<Raw, DaemonError>;
}

#[cfg(feature = "eth")]
impl InjectiveSigner for PrivateKey {
    fn sign_injective(&self, sign_doc: SignDoc) -> Result<Raw, DaemonError> {
        let sign_doc_bytes = sign_doc.clone().into_bytes()?;

        // sign with ethers

        let msg_hash = keccak256(sign_doc_bytes);
        let eth_signature = ethers_signers::Wallet::from_bytes(&self.raw_key())
            .unwrap()
            .sign_hash(msg_hash.into())
            .unwrap();

        let tx_raw: Raw = proto::cosmos::tx::v1beta1::TxRaw {
            body_bytes: sign_doc.body_bytes,
            auth_info_bytes: sign_doc.auth_info_bytes,
            signatures: vec![eth_signature.to_vec()],
        }
        .into();

        Ok(tx_raw)
    }
}
