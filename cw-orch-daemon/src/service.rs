use std::{sync::Arc, task::{Context, Poll}};

use tonic::{body::BoxBody, client::GrpcService, transport::{channel, Channel}, Request};
use tower::{Service, ServiceBuilder};

use crate::DaemonState;

struct DaemonChannel {
    svs: Channel,
}

impl DaemonChannel {
    fn new(channel: Channel) -> Self {
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