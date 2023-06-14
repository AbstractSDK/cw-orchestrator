#![allow(missing_docs)]

use cosmrs::proto::traits::TypeUrl;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectiveEthAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<super::super::cosmos_modules::auth::BaseAccount>,
    #[prost(bytes, tag = "2")]
    pub code_hash: Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectivePubKey {
    #[prost(string, tag = "1")]
    pub key: String,
}

impl TypeUrl for InjectivePubKey{
    const TYPE_URL: &'static str = "/injective.crypto.v1beta1.ethsecp256k1.PubKey";
}
