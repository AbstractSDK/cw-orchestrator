use std::{future::{ready, Ready}, sync::Arc, task::{Context, Poll}, time::Duration};

use tonic::{body::BoxBody, client::GrpcService, transport::{channel, Channel}, Request};
use tower::{retry::{Policy, RetryLayer}, Service, ServiceBuilder};

use crate::DaemonState;

#[derive(Clone)]
pub struct DaemonChannel {
    svs: Channel,
}

impl DaemonChannel {
    pub fn new(channel: Channel) -> Self {
        Self {
            svs: channel
        }
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

    fn retry(&self, _req: &http::Request<BoxBody>, result: Result<&http::Response<hyper::Body>, &E>) -> Option<Self::Future> {
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