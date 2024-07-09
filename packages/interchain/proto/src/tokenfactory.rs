#![allow(non_snake_case)]

use cosmrs::{
    proto::ibc::applications::transfer::v1::MsgTransferResponse, tx::Msg, AccountId, Result,
};

use cw_orch_interchain_core::{
    channel::InterchainChannel, types::IbcTxAnalysis, IbcQueryHandler, InterchainEnv,
    InterchainError,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    MsgCreateDenom, MsgCreateDenomResponse, MsgMint, MsgMintResponse,
};
use tonic::transport::Channel;

use std::str::FromStr;

use cosmrs::Denom;
use cosmwasm_std::Coin;
use cw_orch_core::environment::{CwEnv, TxHandler};
use cw_orch_traits::FullNode;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

use crate::ics20::MsgTransfer;

/// Creates a new denom using the token factory module.
/// This is used mainly for tests, but feel free to use that in production as well
pub fn create_denom<Chain: FullNode>(
    chain: &Chain,
    token_name: &str,
) -> Result<(), <Chain as TxHandler>::Error> {
    let creator = chain.sender_addr().to_string();

    let any = MsgCreateDenom {
        sender: creator,
        subdenom: token_name.to_string(),
    }
    .to_any();

    chain.commit_any::<MsgCreateDenomResponse>(
        vec![cosmrs::Any {
            type_url: any.type_url,
            value: any.value,
        }],
        None,
    )?;

    log::info!("Created denom {}", get_denom(chain, token_name));

    Ok(())
}

/// Gets the denom of a token created by a daemon object
/// This actually creates the denom for a token created by an address (which is here taken to be the daemon sender address)
/// This is mainly used for tests, but feel free to use that in production as well
pub fn get_denom<Chain: CwEnv>(daemon: &Chain, token_name: &str) -> String {
    let sender = daemon.sender_addr().to_string();
    format!("factory/{}/{}", sender, token_name)
}

/// Mints new subdenom token for which the minter is the sender of chain object
/// This mints new tokens to the receiver address
/// This is mainly used for tests, but feel free to use that in production as well
pub fn mint<Chain: FullNode>(
    chain: &Chain,
    receiver: &str,
    token_name: &str,
    amount: u128,
) -> Result<(), <Chain as TxHandler>::Error> {
    let sender = chain.sender_addr().to_string();
    let denom = get_denom(chain, token_name);

    let any = MsgMint {
        sender,
        mint_to_address: receiver.to_string(),
        amount: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
            denom,
            amount: amount.to_string(),
        }),
    }
    .to_any();

    chain.commit_any::<MsgMintResponse>(
        vec![cosmrs::Any {
            type_url: any.type_url,
            value: any.value,
        }],
        None,
    )?;

    log::info!("Minted coins {} {}", amount, get_denom(chain, token_name));

    Ok(())
}

// 1 hour should be sufficient for packet timeout
const TIMEOUT_IN_NANO_SECONDS: u64 = 3_600_000_000_000;

/// Ibc token transfer
/// This allows transfering token over a channel using an interchain_channel object
#[allow(clippy::too_many_arguments)]
pub fn transfer_tokens<Chain: IbcQueryHandler + FullNode, IBC: InterchainEnv<Chain>>(
    origin: &Chain,
    receiver: &str,
    fund: &Coin,
    interchain_env: &IBC,
    ibc_channel: &InterchainChannel<Channel>,
    timeout: Option<u64>,
    memo: Option<String>,
) -> Result<IbcTxAnalysis<Chain>, InterchainError> {
    let chain_id = origin.block_info().unwrap().chain_id;

    let (source_port, _) = ibc_channel.get_ordered_ports_from(&chain_id)?;

    let any = MsgTransfer {
        source_port: source_port.port.to_string(),
        source_channel: source_port.channel.unwrap().to_string(),
        token: Some(cosmrs::Coin {
            amount: fund.amount.u128(),
            denom: Denom::from_str(fund.denom.as_str()).unwrap(),
        }),
        sender: AccountId::from_str(origin.sender_addr().to_string().as_str()).unwrap(),
        receiver: AccountId::from_str(receiver).unwrap(),
        timeout_height: None,
        timeout_revision: None,
        timeout_timestamp: origin.block_info().unwrap().time.nanos()
            + timeout.unwrap_or(TIMEOUT_IN_NANO_SECONDS),
        memo,
    }
    .to_any()
    .unwrap();

    // We send tokens using the ics20 message over the channel that is passed as an argument
    let send_tx = origin
        .commit_any::<MsgTransferResponse>(
            vec![cosmrs::Any {
                type_url: any.type_url,
                value: any.value,
            }],
            None,
        )
        .unwrap();

    // We wait for the IBC tx to stop successfully
    let tx_results = interchain_env
        .await_packets(&source_port.chain_id, send_tx)
        .unwrap();

    Ok(tx_results)
}

const ICS20_CHANNEL_VERSION: &str = "ics20-1";
/// Channel creation between the transfer channels of two blockchains of a starship integration
pub fn create_transfer_channel<Chain: IbcQueryHandler, IBC: InterchainEnv<Chain>>(
    chain1: &str,
    chain2: &str,
    interchain: &IBC,
) -> anyhow::Result<InterchainChannel<<Chain as IbcQueryHandler>::Handler>> {
    let creation = interchain
        .create_channel(
            chain1,
            chain2,
            &PortId::transfer(),
            &PortId::transfer(),
            ICS20_CHANNEL_VERSION,
            Some(cosmwasm_std::IbcOrder::Unordered),
        )
        .unwrap();

    Ok(creation.interchain_channel)
}
