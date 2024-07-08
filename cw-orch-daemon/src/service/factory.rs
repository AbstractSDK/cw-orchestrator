use std::{
    cell::RefCell,
    error::Error,
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use cw_orch_core::environment::{ChainInfoOwned, ChainState};
use tonic::{
    body::BoxBody,
    client::GrpcService,
    transport::{channel, Channel, Endpoint},
    Request,
};
use tower::{
    reconnect::Reconnect,
    retry::{Policy, Retry, RetryLayer},
    Layer, MakeService, Service, ServiceBuilder, ServiceExt,
};

use crate::{DaemonBase, DaemonError, GrpcChannel};

use super::{
    channel::{DaemonChannel, MyRetryPolicy},
    get_channel_creator, NewChannelRequest, Request as ChannelRequest,
};

/// Daemon Service is a wrapper around the [`Reconnect`] layer that is owned by the thread.
pub type DaemonService = Reconnect<Retry<MyRetryPolicy, DaemonChannel>, ()>;

// pub trait DaemonServiceCreation {
//     /// Create a new `Rc<RefCell<Reconnect<...>>>` service for interacting with chain.
//     async fn new_service(&self, chain_info: &ChainInfoOwned) -> Result<DaemonService, DaemonError> {

//         Ok(DaemonService::new(RefCell::new(Reconnect::new::<DaemonChannel, Channel>(DaemonChannelFactory {}, )));
//     }

//     /// Get a new service for interacting with the chain.
//     async fn channel(&self) -> Result<&mut Reconnect<DaemonChannelFactory, Channel>, DaemonError>;
// }

// TODO: take different type than channel
impl<Sender> Service<ChainInfoOwned> for DaemonBase<Sender> {
    type Response = Retry<MyRetryPolicy, DaemonChannel>;
    type Error = Box<dyn Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: ()) -> Self::Future {
        let fut = async move {
            // Create request to get a new channel and re-create the service.
            // TODO: Add a check here to ensure we only try to reconnect # number of times. Otherwise we will never fail.
            let channel_creator = get_channel_creator().await;
            let (reply_tx, reply_rx) = tokio::sync::mpsc::channel(1);

            let request = NewChannelRequest {
                request: ChannelRequest {
                    grpc_urls: self.state().chain_data.as_ref().grpc_urls.clone(),
                    chain_id: self.state().chain_data.as_ref().chain_id.clone(),
                },
                reply_tx,
            };
            let _ = channel_creator.send(request).await;
            let channel = reply_rx.recv().await.unwrap();
            Ok(DaemonChannel::new(request))
        };
        Box::pin(fut)
    }
}
