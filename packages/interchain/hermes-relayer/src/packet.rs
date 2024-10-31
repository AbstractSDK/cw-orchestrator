use crate::core::HermesRelayer;
use cw_orch_core::environment::QuerierGetter;
use cw_orch_daemon::queriers::Ibc;
use cw_orch_interchain_core::env::ChainId;
use ibc_relayer::link::{Link, LinkParameters};
use ibc_relayer_cli::cli_utils::ChainHandlePair;
use ibc_relayer_types::core::{ics04_channel, ics24_host::identifier};
use ibc_relayer_types::core::{
    ics04_channel::packet::Sequence,
    ics24_host::identifier::{ChannelId, PortId},
};

impl HermesRelayer {
    pub fn force_packet_relay(
        &self,
        src_chain: ChainId,
        src_port: PortId,
        src_channel: ChannelId,
        dst_chain: ChainId,
        sequence: Sequence,
    ) {
        self.receive_packet(
            src_chain,
            src_port.clone(),
            src_channel.clone(),
            dst_chain,
            sequence,
        );
        self.ack_packet(
            src_chain,
            src_port.clone(),
            src_channel.clone(),
            dst_chain,
            sequence,
        );
    }

    pub fn receive_packet(
        &self,
        src_chain: ChainId,
        src_port: PortId,
        src_channel: ChannelId,
        dst_chain: ChainId,
        sequence: Sequence,
    ) {
        let config = self.duplex_config(src_chain, dst_chain);
        let chains = ChainHandlePair::spawn(
            &config,
            &identifier::ChainId::from_string(src_chain),
            &identifier::ChainId::from_string(dst_chain),
        )
        .unwrap();

        let opts = LinkParameters {
            src_port_id: src_port.to_string().parse().unwrap(),
            src_channel_id: src_channel.to_string().parse().unwrap(),
            max_memo_size: config.mode.packets.ics20_max_memo_size,
            max_receiver_size: config.mode.packets.ics20_max_receiver_size,

            // Packets are only excluded when clearing
            exclude_src_sequences: vec![],
        };

        self.add_key(&chains.src);
        self.add_key(&chains.dst);

        let link = Link::new_from_opts(chains.src, chains.dst, opts, false, false).unwrap();

        let sequence: u64 = sequence.into();
        let sequence = ics04_channel::packet::Sequence::from(sequence);

        link.relay_recv_packet_and_timeout_messages_with_packet_data_query_height(
            vec![sequence..=sequence],
            None,
        )
        .unwrap();
    }

    pub fn ack_packet(
        &self,
        src_chain: ChainId,
        src_port: PortId,
        src_channel: ChannelId,
        dst_chain: ChainId,
        sequence: Sequence,
    ) {
        let config = self.duplex_config(src_chain, dst_chain);

        let chains = ChainHandlePair::spawn(
            &config,
            &identifier::ChainId::from_string(dst_chain),
            &identifier::ChainId::from_string(src_chain),
        )
        .unwrap();

        let (d, _, _) = self.daemons.get(src_chain).unwrap();

        let ibc: Ibc = d.querier();

        let counterparty = d
            .rt_handle
            .block_on(ibc._channel(src_port.to_string(), src_channel.to_string()))
            .unwrap()
            .counterparty
            .unwrap();

        let opts = LinkParameters {
            src_port_id: counterparty.port_id.to_string().parse().unwrap(),
            src_channel_id: counterparty.channel_id.to_string().parse().unwrap(),
            max_memo_size: config.mode.packets.ics20_max_memo_size,
            max_receiver_size: config.mode.packets.ics20_max_receiver_size,

            // Packets are only excluded when clearing
            exclude_src_sequences: vec![],
        };

        self.add_key(&chains.src);
        self.add_key(&chains.dst);

        let link = Link::new_from_opts(chains.src, chains.dst, opts, false, false).unwrap();

        let sequence: u64 = sequence.into();
        let sequence = ics04_channel::packet::Sequence::from(sequence);

        link.relay_ack_packet_messages_with_packet_data_query_height(
            vec![sequence..=sequence],
            None,
        )
        .unwrap();
    }
}
