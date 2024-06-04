use cw_orch::{
    daemon::networks::ARCHWAY_1,
    environment::{ChainInfo, NetworkInfo},
    prelude::networks::osmosis::OSMOSIS_1,
};
use cw_orch_interchain_core::IbcAckParser;
use cw_orch_interchain_daemon::{ChannelCreationValidator, DaemonInterchainEnv};
use tokio::runtime::Runtime;

pub const NOBLE: NetworkInfo = NetworkInfo {
    chain_name: "noble",
    pub_address_prefix: "noble",
    coin_type: 118,
};
pub const NOBLE_1: ChainInfo = ChainInfo {
    chain_id: "noble-1",
    gas_denom: "uusdc",
    gas_price: 0.1,
    grpc_urls: &["http://noble-grpc.polkachu.com:21590"],
    lcd_url: None,
    fcd_url: None,
    network_info: NOBLE,
    kind: cw_orch::environment::ChainKind::Mainnet,
};

fn follow_by_tx_hash() -> cw_orch::anyhow::Result<()> {
    dotenv::dotenv()?;
    let rt = Runtime::new()?;

    let dst_chain = ARCHWAY_1;
    let src_chain = OSMOSIS_1;

    let interchain = DaemonInterchainEnv::new(
        rt.handle(),
        vec![(src_chain.clone(), None), (dst_chain, None)],
        &ChannelCreationValidator,
    )?;

    let mut result = interchain
        .wait_ibc_from_txhash(
            src_chain.chain_id,
            "D2C5459C54B394C168B8DFA214670FF9E2A0349CCBEF149CF5CB508A5B3BCB84".to_string(),
        )?
        .analyze()?;

    result.find_and_pop(&IbcAckParser::ics20_ack)?;

    let mut result = interchain
        .wait_ibc_from_txhash(
            src_chain.chain_id,
            "D2C5459C54B394C168B8DFA214670FF9E2A0349CCBEF149CF5CB508A5B3BCB84".to_string(),
        )?
        .analyze()?;
    result
        .find_and_pop(&IbcAckParser::polytone_ack)
        .unwrap_err();

    let mut result = interchain
        .wait_ibc_from_txhash(
            src_chain.chain_id,
            "D2C5459C54B394C168B8DFA214670FF9E2A0349CCBEF149CF5CB508A5B3BCB84".to_string(),
        )?
        .analyze()?;
    result.find_and_pop(&IbcAckParser::ics004_ack).unwrap_err();

    Ok(())
}

fn main() {
    env_logger::init();
    follow_by_tx_hash().unwrap();
}
