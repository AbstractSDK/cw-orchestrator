use crate::client_types::terra_decimal_format;
use cosmrs::Coin as CosmCoin;
use regex::Regex;

use rust_decimal_macros::dec;
// use rust_decimal::prelude::*;
use rust_decimal::{prelude::FromPrimitive, Decimal};

use crate::error::CosmScriptError;
//use base64::{ToBase64, STANDARD};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// The primary way to denote currency
/// NB: Internally everything is represented by their uXXX format.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coin {
    #[allow(missing_docs)]
    #[serde(with = "terra_decimal_format")]
    pub amount: Decimal,
    /// the coin type. in uXXX format
    pub denom: String,
}

impl Coin {
    /// Standard Coin creation
    pub fn create(denom: &str, amount: Decimal) -> Coin {
        Coin {
            denom: denom.to_string(),
            amount,
        }
    }
    /// Parse the string "nnnnnXXXX" format where XXXX is the coin type
    pub fn parse(str: &str) -> Result<Option<Coin>, CosmScriptError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+[.]?\d*)([a-zA-Z]+)$").unwrap();
            static ref RE_IBC: Regex = Regex::new(r"^(\d+[.]?\d*)(ibc/[a-fA-F0-9]+)$").unwrap();
        }
        //    let RE: Regex = Regex::new(r"^(\d+)(\s+)$").unwrap();

        match RE.captures(str) {
            Some(cap) => Ok(Some(Coin::create(
                &cap[2],
                cap.get(1).unwrap().as_str().parse::<Decimal>()?,
            ))),
            None => match RE_IBC.captures(str) {
                Some(cap) => Ok(Some(Coin::create(
                    &cap[2],
                    cap.get(1).unwrap().as_str().parse::<Decimal>()?,
                ))),
                None => Ok(None),
            },
        }
    }
    /// this will take a comma delimited string of coins and return a sorted (by denom) vector of coins
    /// eg "22.707524482460197756uaud,21.882510617180501989ucad,16.107413560222631626uchf,114.382279464849248732ucny,14.594888140543189388ueur,12.689498975492463452ugbp,136.932658449160933002uhkd,1315.661396873891976912uinr,1917.803659404458501345ujpy,20710.846165266109229516ukrw,50292.255931832196576203umnt,12.276992042852615569usdr,23.395036036859944228usgd,0.0uthb,17.639582167170638049uusd"
    ///
    pub fn parse_coins(str: &str) -> Result<Vec<Coin>, CosmScriptError> {
        let vec_res_opt_coins = str
            .split(',')
            .map(Coin::parse)
            .collect::<Vec<Result<Option<Coin>, CosmScriptError>>>();
        let mut coins: Vec<Coin> = Vec::with_capacity(vec_res_opt_coins.len());
        for vroc in vec_res_opt_coins {
            let coin_opt = vroc.map_err(|_source| CosmScriptError::CoinParseErrV {
                parse: str.parse().unwrap(),
            })?;

            match coin_opt {
                None => {
                    return Err(CosmScriptError::CoinParseErr(str.parse().unwrap()));
                }
                Some(coin) => {
                    coins.push(coin);
                }
            };
        }
        coins.sort_by(|a, b| a.denom.cmp(&b.denom));
        Ok(coins)
    }
}
impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.amount == dec!(0.0) {
            write!(f, "0.0{}", self.denom)
        } else {
            write!(f, "{:.}{}", self.amount, self.denom)
        }
    }
}
impl PartialEq for Coin {
    fn eq(&self, other: &Self) -> bool {
        self.denom == other.denom && self.amount == other.amount
    }
}

impl From<&CosmCoin> for Coin {
    fn from(coin: &CosmCoin) -> Self {
        Self {
            amount: Decimal::from_u64(u64::from_str(&coin.amount.to_string()).unwrap()).unwrap(),
            denom: coin.denom.to_string(),
        }
    }
}
#[cfg(test)]
mod tst {
    use super::*;
    #[test]
    pub fn test_coin() -> anyhow::Result<()> {
        let c = Coin::create("uluna", dec!(1000.0));
        assert_eq!(c.amount, dec!(1000.0));
        assert_eq!(c.denom, "uluna");
        let d = Coin::parse("1000uluna")?;
        match d {
            Some(c) => {
                assert_eq!(c.amount, dec!(1000.0));
                assert_eq!(c.denom, "uluna");
            }
            None => {
                assert!(false)
            }
        }

        let e = Coin::parse("1000")?;
        assert!(e.is_none());
        let f = Coin::parse("")?;
        assert!(f.is_none());
        Ok(())
    }
    #[test]
    pub fn test_rate() -> anyhow::Result<()> {
        let d = Coin::parse("50292.255931832196576203umnt")?;
        match d {
            Some(c) => {
                assert_eq!(c.denom, "umnt");
                assert_eq!(c.amount, dec!(50292.255931832196576203));
                assert_eq!(c.to_string(), "50292.255931832196576203umnt");
            }
            None => assert!(false),
        }
        let e = Coin::parse("0umnt")?;
        match e {
            Some(c) => {
                assert_eq!(c.denom, "umnt");
                assert_eq!(c.amount, dec!(0.0));
                assert_eq!(c.to_string(), "0.0umnt");
            }
            None => {
                eprintln!("Regex not working for whole numbers?");
                assert!(false)
            }
        }
        Ok(())
    }
    #[test]
    fn test_coins() -> anyhow::Result<()> {
        let exchange_rates3="22.707524482460197756uaud,21.882510617180501989ucad,16.107413560222631626uchf,114.382279464849248732ucny,14.594888140543189388ueur,12.689498975492463452ugbp,136.932658449160933002uhkd,1315.661396873891976912uinr,1917.803659404458501345ujpy,20710.846165266109229516ukrw,50292.255931832196576203umnt,12.276992042852615569usdr,23.395036036859944228usgd,0.0uthb,17.639582167170638049uusd";
        let vec = Coin::parse_coins(exchange_rates3)?;
        assert_eq!(vec.len(), 15);
        let c = vec.get(2).unwrap();
        assert_eq!(c.denom, "uchf");
        assert_eq!(c.amount, dec!(16.107413560222631626));
        let exchange_rates_unsorted="16.107413560222631626uchf,114.382279464849248732ucny,14.594888140543189388ueur,12.689498975492463452ugbp,136.932658449160933002uhkd,1315.661396873891976912uinr,1917.803659404458501345ujpy,20710.846165266109229516ukrw,50292.255931832196576203umnt,12.276992042852615569usdr,23.395036036859944228usgd,0.0uthb,21.882510617180501989ucad,17.639582167170638049uusd,22.707524482460197756uaud";
        let vec2 = Coin::parse_coins(exchange_rates_unsorted)?;
        assert_eq!(vec2.len(), 15);
        let c = vec2.get(2).unwrap();
        assert_eq!(c.denom, "uchf");
        assert_eq!(c.amount, dec!(16.107413560222631626));
        for i in 0..vec2.len() {
            let c_v1 = vec.get(i).unwrap();
            let c_v2 = vec2.get(i).unwrap();

            assert_eq!(c_v1, c_v2);
        }

        Ok(())
    }
    #[test]
    fn test_ibc_coins() -> anyhow::Result<()> {
        let c = Coin::parse("566.750000000000000000ibc/EB2CED20AB0466F18BE49285E56B31306D4C60438A022EA995BA65D5E3CF7E09")?;
        match c {
            Some(c) => {
                assert_eq!(
                    c.denom,
                    "ibc/EB2CED20AB0466F18BE49285E56B31306D4C60438A022EA995BA65D5E3CF7E09"
                );
                assert_eq!(c.amount, dec!(566.75));
            }
            None => assert!(false),
        }
        let ibc_coin_string="566.750000000000000000ibc/EB2CED20AB0466F18BE49285E56B31306D4C60438A022EA995BA65D5E3CF7E09,26762036.250000000000000000ukrw,2545.950000000000000000uluna,528551.000000000000000000uusd";
        let vec = Coin::parse_coins(ibc_coin_string)?;
        assert_eq!(vec.len(), 4);

        Ok(())
    }
}
