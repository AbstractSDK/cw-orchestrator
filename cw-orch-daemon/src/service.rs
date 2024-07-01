use std::{
    cell::RefCell, error::Error, future::{ready, Future, Ready}, pin::Pin, rc::Rc, sync::Arc, task::{Context, Poll}, time::Duration
};

use cw_orch_core::environment::ChainInfoOwned;
use tonic::{
    body::BoxBody,
    client::GrpcService,
    transport::{channel, Channel, Endpoint},
    Request,
};
use tower::{
    reconnect::Reconnect, retry::{Policy, Retry, RetryLayer}, Layer, MakeService, Service, ServiceBuilder, ServiceExt
};

use crate::{DaemonError, GrpcChannel};

/// Daemon Service is a wrapper around the [`Reconnect`] layer that can be shared within a thread. 
/// This allows our services to scale between different threads while still being error-tolerant.
/// A signing daemon will have two DaemonService instances, one for transactions and one for queries.
pub type DaemonService = Rc<RefCell<Reconnect<DaemonChannelFactory, Channel>>>;

pub trait DaemonServiceCreation {
    /// Create a new `Rc<RefCell<Reconnect<...>>>` service for interacting with chain.
    async fn new_service(&self, chain_info: &ChainInfoOwned) -> Result<DaemonService, DaemonError> {
        let channel = GrpcChannel::connect(&chain_info.grpc_urls, &chain_info.chain_id).await?;
        Ok(DaemonService::new(RefCell::new(Reconnect::new::<DaemonChannel, Channel>(DaemonChannelFactory {}, channel))));
    }

    /// Get a new service for interacting with the chain.
    async fn channel(&self) -> Result<&mut Reconnect<DaemonChannelFactory, Channel>, DaemonError>;
}

#[derive(Clone)]
pub struct DaemonChannel {
    // Service that retries on failures
    pub(crate) svs: Retry<MyRetryPolicy, Channel>,
}

impl DaemonChannel {
    pub fn new(channel: Channel) -> Self {
        // Create the Reconnect layer with the factory
        let retry_policy = MyRetryPolicy {
            max_retries: 3,                  // Maximum number of retries
            backoff: Duration::from_secs(1), // Backoff duration
        };
        let retry_layer = RetryLayer::new(retry_policy);
        let a: Retry<MyRetryPolicy, Channel> = ServiceBuilder::new()
            .layer(retry_layer)
            .service(channel);
        Self { svs: a }
    }
}

impl Service<http::Request<BoxBody>> for DaemonChannel {
    type Response = http::Response<hyper::Body>;
    type Error = tonic::transport::Error;
    type Future = channel::ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Service::poll_ready(&mut self.svs, cx)
    }

    fn call(&mut self, request: http::Request<BoxBody>) -> Self::Future {
        Service::call(&mut self.svs, request)
    }
}

pub struct DaemonChannelFactory {}

// TODO: take different type than channel
impl Service<Channel> for DaemonChannelFactory {
    type Response = Retry<MyRetryPolicy, DaemonChannel>;
    type Error = Box<dyn Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Channel) -> Self::Future {
        let fut = async move {
            Ok(DaemonChannel::new(request))
        };
        Box::pin(fut)
    }
}

/// Retry policy that retries on all errors up to a maximum number of retries.
#[derive(Debug, Clone)]
pub struct MyRetryPolicy {
    pub max_retries: usize,
    pub backoff: Duration,
}

impl<E> Policy<http::Request<BoxBody>, http::Response<hyper::Body>, E> for MyRetryPolicy
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Future = Ready<Self>;

    fn retry(
        &self,
        _req: &http::Request<BoxBody>,
        result: Result<&http::Response<hyper::Body>, &E>,
    ) -> Option<Self::Future> {
        if self.max_retries > 0 && result.is_err() {
            Some(ready(MyRetryPolicy {
                max_retries: self.max_retries - 1,
                backoff: self.backoff,
            }))
        } else {
            None
        }
    }

    fn clone_request(&self, req: &http::Request<BoxBody>) -> Option<http::Request<BoxBody>> {
        None
    }
}
