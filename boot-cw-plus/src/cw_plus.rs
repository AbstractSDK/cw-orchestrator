use crate::*;
use boot_core::*;

/// Wrapper around the CwPlus contracts for use in cw-multi-tests and deployments.
pub struct CwPlus<Chain: CwEnv> {
    pub cw1_whitelist: Cw1Whitelist<Chain>,
    pub cw1_subkeys: Cw1Subkeys<Chain>,
    pub cw3_fixed_multisig: Cw3FixedMultisig<Chain>,
    pub cw3_flex_multisig: Cw3FlexMultisig<Chain>,
    pub cw4_group: Cw4Group<Chain>,
    pub cw4_stake: Cw4Stake<Chain>,
    pub cw20_base: Cw20Base<Chain>,
    pub cw20_ics20: Cw20Ics20<Chain>,
}

impl<Chain: CwEnv> Deploy<Chain> for CwPlus<Chain> {
    type Error = BootError;
    type DeployData = Empty;

    /// Deploy all cw-plus contracts to the given chain.
    fn deploy_on(chain: Chain, _data: Empty) -> Result<Self, BootError> {
        let mut cw20_base = Cw20Base::new(CW20_BASE, chain.clone());
        cw20_base.upload()?;
        let mut cw1_whitelist = Cw1Whitelist::new(CW1_WHITELIST, chain.clone());
        cw1_whitelist.upload()?;
        let mut cw1_subkeys = Cw1Subkeys::new(CW1_SUBKEYS, chain.clone());
        cw1_subkeys.upload()?;
        let mut cw20_ics20 = Cw20Ics20::new(CW20_ICS20, chain.clone());
        cw20_ics20.upload()?;
        let mut cw3_fixed_multisig = Cw3FixedMultisig::new(CW3_FIXED_MULTISIG, chain.clone());
        cw3_fixed_multisig.upload()?;
        let mut cw3_flex_multisig = Cw3FlexMultisig::new(CW3_FLEX_MULTISIG, chain.clone());
        cw3_flex_multisig.upload()?;
        let mut cw4_group = Cw4Group::new(CW4_GROUP, chain.clone());
        cw4_group.upload()?;
        let mut cw4_stake = Cw4Stake::new(CW4_STAKE, chain);
        cw4_stake.upload()?;

        Ok(Self {
            cw20_base,
            cw1_whitelist,
            cw1_subkeys,
            cw20_ics20,
            cw3_fixed_multisig,
            cw3_flex_multisig,
            cw4_group,
            cw4_stake,
        })
    }
    /// Load CwPlus contracts from the chain.
    fn load_from(chain: Chain) -> Result<Self, BootError> {
        let cw20_base = Cw20Base::new(CW20_BASE, chain.clone());
        let cw1_whitelist = Cw1Whitelist::new(CW1_WHITELIST, chain.clone());
        let cw1_subkeys = Cw1Subkeys::new(CW1_SUBKEYS, chain.clone());
        let cw20_ics20 = Cw20Ics20::new(CW20_ICS20, chain.clone());
        let cw3_fixed_multisig = Cw3FixedMultisig::new(CW3_FIXED_MULTISIG, chain.clone());
        let cw3_flex_multisig = Cw3FlexMultisig::new(CW3_FLEX_MULTISIG, chain.clone());
        let cw4_group = Cw4Group::new(CW4_GROUP, chain.clone());
        let cw4_stake = Cw4Stake::new(CW4_STAKE, chain);

        Ok(Self {
            cw20_base,
            cw1_whitelist,
            cw1_subkeys,
            cw20_ics20,
            cw3_fixed_multisig,
            cw3_flex_multisig,
            cw4_group,
            cw4_stake,
        })
    }
}
