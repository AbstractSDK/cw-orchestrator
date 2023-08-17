use base64::{engine::general_purpose, Engine as _};
use cosmrs::{
    proto::cosmos::bank::v1beta1::{QueryAllBalancesRequest, QueryAllBalancesResponse},
    rpc::{endpoint::abci_query, Client, HttpClient},
    tx::MessageExt,
};
use hex::encode;
use prost::Message;
use tokio::runtime::Runtime;

#[test]
fn temp_test() {
    // Necessary variable for querying
    let rt = Runtime::new().unwrap();
    let client = HttpClient::new("https://rpc.osmosis.zone").unwrap();

    // Query content
    let address = "osmo126pr9qp44aft4juw7x4ev4s2qdtnwe38jzwunec9pxt5cpzaaphqyagqpu".to_string();
    let request = QueryAllBalancesRequest {
        address,
        pagination: None,
    };
    let any = request.to_bytes().unwrap();

    // Querying
    let response = rt
        .block_on(client.abci_query(
            Some("/cosmos.bank.v1beta1.Query/AllBalances".to_string()),
            any,
            None,
            true,
        ))
        .unwrap();

    // Analysing the response
    let balance_response = QueryAllBalancesResponse::decode(response.value.as_slice()).unwrap();

    // Printing the response
    panic!("{:?}", response);
}
