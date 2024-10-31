use cw_orch_core::environment::ChainInfoOwned;
use hdpath::{Purpose, StandardHDPath};
use ibc_relayer::{
    config::AddressType,
    keyring::{AnySigningKeyPair, Secp256k1KeyPair, SigningKeyPair},
};

pub fn restore_key(
    mnemonic: String,
    hdpath_index: u32,
    chain_data: &ChainInfoOwned,
) -> anyhow::Result<AnySigningKeyPair> {
    let hdpath = StandardHDPath::new(
        Purpose::Pubkey,
        chain_data.network_info.coin_type,
        0,
        0,
        hdpath_index,
    );

    let key_pair = Secp256k1KeyPair::from_mnemonic(
        &mnemonic,
        &hdpath,
        &AddressType::Cosmos,
        &chain_data.network_info.pub_address_prefix,
    )?;

    Ok(key_pair.into())
}
