#[derive(Default, PartialEq, Eq, Debug, Clone, derive_more::FromStr)]
pub struct CliCoins(pub cosmwasm_std::Coins);

impl TryFrom<&CliCoins> for Vec<cosmrs::Coin> {
    type Error = color_eyre::Report;

    fn try_from(value: &CliCoins) -> Result<Self, Self::Error> {
        value
            .0
            .iter()
            .map(|cosmwasm_std::Coin { amount, denom }| {
                Ok(cosmrs::Coin {
                    amount: amount.u128(),
                    denom: denom.parse()?,
                })
            })
            .collect()
    }
}

impl From<CliCoins> for Vec<cosmwasm_std::Coin> {
    fn from(value: CliCoins) -> Self {
        value.0.into()
    }
}

impl std::fmt::Display for CliCoins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl interactive_clap::ToCli for CliCoins {
    type CliVariant = CliCoins;
}
