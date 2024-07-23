/// macro for constructing and performing a query on a CosmosSDK module.
#[macro_export]
macro_rules! cosmos_query {
    ($self:ident, $module:ident, $func_name:ident, $request_type:ident { $($field:ident : $value:expr),* $(,)?  }) => {
        {
        use $crate::cosmos_modules::$module::{
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