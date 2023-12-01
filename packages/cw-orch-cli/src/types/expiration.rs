use std::str::FromStr;

use cosmwasm_std::Timestamp;
use cw_utils::Expiration;

#[derive(Debug, Default, Clone, derive_more::AsRef, derive_more::From, derive_more::Into)]
#[as_ref(forward)]
pub struct CliExpiration(pub Expiration);

impl std::fmt::Display for CliExpiration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl interactive_clap::ToCli for CliExpiration {
    type CliVariant = CliExpiration;
}

impl FromStr for CliExpiration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if s == "never" {
            return Ok(Self(Expiration::Never {}));
        }
        let (t, num) = s.split_once(':').ok_or("Incorrect expiration format")?;
        let expiration = match t {
            "height" => {
                let height: u64 = num.parse().map_err(|_| "Failed to parse height")?;
                Expiration::AtHeight(height)
            }
            "time" => {
                let timestamp: u64 = num.parse().map_err(|_| "Failed to parse timestamp")?;
                Expiration::AtTime(Timestamp::from_nanos(timestamp))
            }
            _ => return Err("Unknown expiration type".to_owned()),
        };
        Ok(CliExpiration(expiration))
    }
}
