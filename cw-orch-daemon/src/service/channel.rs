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

use cw_orch_core::environment::ChainInfoOwned;
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

use crate::{DaemonError, GrpcChannel};

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
        let a: Retry<MyRetryPolicy, Channel> =
            ServiceBuilder::new().layer(retry_layer).service(channel);
        Self { svs: a }
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
