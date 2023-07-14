use std::time::Duration;

pub async fn sleep(duration: Duration) {
    #[cfg(feature = "tokio-runtime")]
    tokio::time::sleep(duration).await;
    #[cfg(feature = "wasm-runtime")]
    wasm_timer::Delay::new(duration).await;
}
