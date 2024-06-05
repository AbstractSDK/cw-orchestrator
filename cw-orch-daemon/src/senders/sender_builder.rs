use cw_orch_core::environment::ChainInfoOwned;
use tonic::transport::Channel;

use super::{base_sender::SenderOptions, sender_trait::SenderTrait};

pub trait SenderBuilder {
    type Sender: SenderTrait;

    fn build(
        chain_info: ChainInfoOwned,
        grpc_channel: Channel,
        sender_options: SenderOptions,
    ) -> Result<Self::Sender, <Self::Sender as SenderTrait>::Error>;
}
