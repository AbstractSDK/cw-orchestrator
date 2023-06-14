


#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectiveEthAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<super::super::cosmos_modules::auth::BaseAccount>,
    #[prost(bytes, tag = "2")]
    pub code_hash: Vec<u8>
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectivePubKey {
    #[prost(bytes, tag = "1")]
    pub key: Vec<u8>,
}