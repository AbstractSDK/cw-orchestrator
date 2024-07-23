
/// macro for constructing and performing a query on a CosmosSDK module.
#[macro_export]
macro_rules! cosmos_rpc_query {
    ($self:ident, $module:ident, $type_url:literal, $request_type:ident { $($field:ident : $value:expr),* $(,)?  }, $request_resp:ident, ) => {
    {
        use $crate::cosmos_modules::$module::{
            $request_resp, $request_type,
        };
        use ::cosmrs::rpc::Client;
        use ::cosmrs::tx::MessageExt;
        use ::prost::Message;

        let request = $request_type { $($field : $value),* };
        let response = $self.client.abci_query(
            Some($type_url.to_string()),
            request.to_bytes()?,
            None,
            // Don't request proof, we don't need it
            false,
        ).await?;
        let decoded_response = $request_resp::decode(response.value.as_slice())?;
        ::log::trace!(
            "cosmos_query: {:?} resulted in: {:?}",
            request,
            decoded_response
        );

        decoded_response
    }
};
}
