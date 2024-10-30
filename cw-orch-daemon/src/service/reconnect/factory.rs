use std::{future::Future, pin::Pin};

use tower::Service;

use crate::{DaemonError, GrpcChannel, TowerChannel};

pub type ChannelCreationArgs = (Vec<String>, String);
#[derive(Clone)]
pub struct ChannelFactory {}

impl Service<ChannelCreationArgs> for ChannelFactory {
    type Response = TowerChannel;

    type Error = DaemonError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: ChannelCreationArgs) -> Self::Future {
        Box::pin(async move { GrpcChannel::get_channel(req.0.as_ref(), &req.1).await })
    }
}
