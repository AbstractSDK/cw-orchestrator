use tonic::transport::Channel;

pub trait QuerierTrait {
    fn channel(&self) -> Channel;
}

impl QuerierTrait for () {
    fn channel(&self) -> Channel {
        todo!()
    }
}
