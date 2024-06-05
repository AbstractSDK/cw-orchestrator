use cw_orch_starship::client::StarshipClient;

#[tokio::main]
async fn main() {
    let starship = StarshipClient::new_async(None).await.unwrap();

    starship
        .create_channel(
            "juno-1",
            "osmosis-1",
            "a",
            "b",
            "gg",
            Some(cosmwasm_std::IbcOrder::Unordered),
        )
        .await
        .unwrap();
}
