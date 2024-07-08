mod channel;
mod factory;

use channel::DaemonChannelFactory;
use http::Uri;
use std::sync::Arc;
use tokio::sync::{mpsc, OnceCell};
use tokio::task;
use tonic::transport::{Channel, Endpoint};
use tower::reconnect::Reconnect;

use crate::GrpcChannel;

static SERVICE_FACTORY: OnceCell<Arc<mpsc::Sender<NewChannelRequest>>> = OnceCell::const_new();

#[derive(Debug)]
struct NewChannelRequest {
    request: Request,
    reply_tx: mpsc::Sender<Channel>,
}

#[derive(Debug, Clone)]
pub struct Request {
    grpc_urls: Vec<String>,
    chain_id: String,
}

pub struct Response {}

#[derive(Debug)]
pub struct ChannelFactory {}

impl ChannelFactory {
    pub async fn create_channel(
        &self,
        request: Request,
    ) -> Result<Channel, tonic::transport::Error> {
        // TODO: move channel creation here, find endpoint with best connection (lowest latency / highest rate-limit)
        // And round-robin new connections
        let channel = GrpcChannel::connect(&chain_info.grpc_urls, &chain_info.chain_id).await?;
        Ok(Response)
    }
}

async fn get_channel_creator() -> Arc<mpsc::Sender<NewChannelRequest>> {
    SERVICE_FACTORY
        .get_or_init(|| async {
            let (tx, mut rx) = mpsc::channel::<NewChannelRequest>(32);
            let service_creator = Arc::new(tx.clone());

            task::spawn(async move {
                let factory = ServiceFactory {};
                while let Some(request) = rx.recv().await {
                    let response = factory.create_channel(request).await;
                    let _ = request.reply_tx.send(response).await;
                }
            });

            service_creator
        })
        .await
        .clone()
}
