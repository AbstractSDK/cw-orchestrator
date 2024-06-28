use std::{
    error::Error,
    future::{ready, Future, Ready},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use tonic::{
    body::BoxBody,
    client::GrpcService,
    transport::{channel, Channel},
    Request,
};
use tower::{
    retry::{Policy, RetryLayer}, Layer, MakeService, Service, ServiceBuilder
};

use crate::DaemonState;

#[derive(Clone)]
pub struct DaemonChannel {
    pub (crate) svs: Channel,
}

impl DaemonChannel {
    pub fn new(channel: Channel) -> Self {
        Self { svs: channel }
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

// Is automatically implemented by Tonic !
// impl GrpcService<tonic::body::BoxBody> for DaemonChannel {
// }

pub struct DaemonChannelFactory {}

impl Layer<Channel> for DaemonChannel {
    type Service = DaemonChannel;

    fn layer(&self, inner: Channel) -> Self::Service {
        DaemonChannel::new(inner)
    }
}

impl Service<String> for DaemonChannelFactory {
    type Response = DaemonChannel;
    type Error = Box<dyn Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: String) -> Self::Future {
        let endpoint = request;
        let fut = async move {
            let channel = Channel::from_shared(endpoint)
                .expect("valid URI")
                .connect()
                .await
                .map_err(|e| e.into())
                .map(|channel| DaemonChannel::new(channel));
            channel
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
