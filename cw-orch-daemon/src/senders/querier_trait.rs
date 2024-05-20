use tonic::transport::Channel;

pub trait QuerierTrait: Clone {
    type QuerierBuilder;
    fn channel(&self) -> Channel;
}

impl QuerierTrait for () {
    type QuerierBuilder = ();
    fn channel(&self) -> Channel {
        todo!()
    }
}
