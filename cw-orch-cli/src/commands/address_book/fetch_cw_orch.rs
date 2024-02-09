use crate::types::address_book::{self, select_alias};

use super::AddresBookContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = AddresBookContext)]
#[interactive_clap(output_context = FetchAddressesOutput)]
pub struct FetchAddresses {}

#[derive(Debug, strum::EnumDiscriminants, Clone)]
#[strum_discriminants(derive(strum::EnumMessage, strum::EnumIter))]
pub enum DuplicatesStrategy {
    #[strum_discriminants(strum(message = "Ask every time"))]
    Ask,
    #[strum_discriminants(strum(message = "Skip Duplicates"))]
    Skip,
    #[strum_discriminants(strum(message = "Override Duplicates"))]
    Override,
}

pub struct FetchAddressesOutput;

impl FetchAddressesOutput {
    fn from_previous_context(
        previous_context: AddresBookContext,
        scope: &<FetchAddresses as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let duplicate_strategy = DuplicatesStrategy::Ask;
        let state_file = cw_orch_core::env::CwOrchEnvVars::load()?.state_file;


        let chain = previous_context.chain;
        todo!();

        Ok(FetchAddressesOutput)
    }
}
