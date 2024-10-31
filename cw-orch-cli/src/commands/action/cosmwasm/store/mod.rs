use crate::{commands::action::CosmosContext, types::keys::seed_phrase_for_id};

use cw_orch::prelude::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = StoreWasmOutput)]
/// Execute contract method
pub struct StoreContractCommands {
    /// Input path to the wasm
    wasm_path: crate::types::PathBuf,
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
}

impl StoreContractCommands {
    fn input_signer(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct StoreWasmOutput;

impl StoreWasmOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<StoreContractCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let wasm_path = WasmPath::new(&scope.wasm_path)?;

        let seed = seed_phrase_for_id(&scope.signer)?;
        let daemon = chain.daemon(seed)?;

        let resp = daemon
            .rt_handle
            .block_on(daemon.sender().upload_wasm(wasm_path))?;
        let code_id = resp.uploaded_code_id().unwrap();
        println!("code_id: {code_id}");

        Ok(StoreWasmOutput)
    }
}
