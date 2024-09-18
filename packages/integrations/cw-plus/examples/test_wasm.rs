use cw_orch::daemon::networks::LOCAL_JUNO;
use cw_orch::prelude::*;
use cw_plus_orch::cw1_subkeys::Cw1SubKeys;
use cw_plus_orch::cw1_whitelist::Cw1Whitelist;
use cw_plus_orch::cw20_base::Cw20Base;
use cw_plus_orch::cw20_ics20::Cw20Ics20;
use cw_plus_orch::cw3_fixed_multisig::Cw3FixedMultisig;
use cw_plus_orch::cw3_flex_multisig::Cw3FlexMultisig;
use cw_plus_orch::cw4_group::Cw4Group;
use cw_plus_orch::cw4_stake::Cw4Stake;

fn main() -> cw_orch::anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();

    let daemon = Daemon::builder(LOCAL_JUNO).build()?;

    Cw1SubKeys::new("cw1_subkeys", daemon.clone()).upload()?;
    Cw1Whitelist::new("cw1_whitelist", daemon.clone()).upload()?;
    Cw3FixedMultisig::new("cw3_fixed_multisig", daemon.clone()).upload()?;
    Cw3FlexMultisig::new("cw3_flex_multisig", daemon.clone()).upload()?;
    Cw4Group::new("cw4_group", daemon.clone()).upload()?;
    Cw4Stake::new("cw4_stake", daemon.clone()).upload()?;
    Cw20Base::new("cw20_base", daemon.clone()).upload()?;
    Cw20Ics20::new("cw20_ics20", daemon.clone()).upload()?;

    Ok(())
}
