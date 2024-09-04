use crate::{commands::action::CosmosContext, types::keys::seed_phrase_for_id};

use color_eyre::eyre::Context;
use cw_orch::{daemon::TxSender, prelude::*};

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
        let wasm_byte_code = std::fs::read(&scope.wasm_path).wrap_err(format!(
            "Failed to open or read the file: {}",
            scope.wasm_path.0.display()
        ))?;

        let seed = seed_phrase_for_id(&scope.signer)?;
        let daemon = chain.daemon(seed)?;

        let upload_msg = cosmrs::cosmwasm::MsgStoreCode {
            sender: daemon.sender().account_id(),
            wasm_byte_code,
            instantiate_permission: None,
        };
        let resp = daemon
            .rt_handle
            .block_on(daemon.sender().commit_tx(vec![upload_msg], None))?;
        let code_id = resp.uploaded_code_id().unwrap();
        println!("code_id: {code_id}");

        Ok(StoreWasmOutput)
    }
}

// TODO: the dream here to use Uploadable instead
// fn uploadable_from_path(wasm_path: WasmPath) -> impl Uploadable {
//     struct Placeholder {
//         wasm_path: WasmPath,
//     }
//     impl Uploadable for Placeholder {
//         fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
//             // having &self.wasm_path instead would solve the issue
//             wasm_path
//         }
//     }
//     Placeholder { wasm_path }
// }
