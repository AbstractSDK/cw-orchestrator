#![allow(missing_docs)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerraRustScriptError {
    #[error("Reqwest HTTP(s) Error")]
    ReqwestError(#[from] ::reqwest::Error),
    #[error("JSON Conversion Error")]
    SerdeJson(#[from] ::serde_json::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    IOErr(#[from] ::std::io::Error),
    #[error(transparent)]
    Secp256k1(#[from] ::secp256k1::Error),
    #[error(transparent)]
    VarError(#[from] ::std::env::VarError),

    #[error("Terra `{0}` CLI Error")]
    Terra(String),

    #[error("TerraRustAPIError Error")]
    TerraRustAPIError(#[from] ::terra_rust_api::errors::TerraRustAPIError),
    #[error("Bech32 Decode Error")]
    Bech32DecodeErr,
    #[error("Bech32 Decode Error: Key Failed prefix {0} or length {1} Wanted:{2}/{3}")]
    Bech32DecodeExpanded(String, usize, String, usize),
    #[error("Mnemonic - Bad Phrase")]
    Phrasing,
    #[error("Mnemonic - Missing Phrase")]
    MissingPhrase,
    #[error("Bad Implementation. Missing Component")]
    Implementation,

    #[error("83 length-missing SECP256K1 prefix")]
    ConversionSECP256k1,
    #[error("82 length-missing ED25519 prefix")]
    ConversionED25519,
    #[error("Expected Key length of 82 or 83 length was {0}")]
    ConversionLength(usize),
    #[error("Expected Key length of 40 length was {0}")]
    ConversionLengthED25519Hex(usize),
    #[error("Expected ED25519 key of length 32 with a BECH32 ED25519 prefix of 5 chars - Len {0} - Hex {1}")]
    ConversionPrefixED25519(usize, String),
    #[error("Can't call Transactions without some gas rules")]
    NoGasOpts,
    #[error("Can't parse `{parse}` into a coin")]
    CoinParseErrV { parse: String },
    #[error("Can't parse `{0}` into a coin")]
    CoinParseErr(String),
    #[error("TX submit returned `{0}` - {1} '{2}'")]
    TxResultError(usize, String, String),
    #[error("No price found for Gas using denom {0}")]
    GasPriceError(String),
    #[error("Attempting to fetch validator set in parts, and failed Height mismatch {0} {1}")]
    TendermintValidatorSet(u64, u64),
    #[error("Transaction {0} not found after {1} attempts")]
    TXNotFound(String, usize),
    #[error("unknown Terra-Rust API error")]
    Unknown,
    #[error("Generic Error {0}")]
    StdErr(String),

    #[error("Contract address for {0} not found in file")]
    AddrNotInFile(String),
}
