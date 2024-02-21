use color_eyre::eyre::Context;
use cw_orch::{
    daemon::CosmTxResponse,
    prelude::{DaemonAsync, IndexResponse},
    tokio::runtime::Runtime,
};

use crate::{commands::action::CosmosContext, log::LogOutput, types::keys::seed_phrase_for_id};

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
        let seed = seed_phrase_for_id(&scope.signer)?;
        let wasm_byte_code = std::fs::read(&scope.wasm_path).wrap_err(format!(
            "Failed to open or read the file: {}",
            scope.wasm_path.0.display()
        ))?;

        let rt = Runtime::new()?;
        let resp = rt.block_on(async {
            let daemon = DaemonAsync::builder()
                .chain(chain)
                .mnemonic(seed)
                .build()
                .await?;

            let exec_msg = cosmrs::cosmwasm::MsgStoreCode {
                sender: daemon.sender.pub_addr()?,
                wasm_byte_code,
                instantiate_permission: None,
            };
            let resp = daemon.sender.commit_tx(vec![exec_msg], None).await?;
            color_eyre::Result::<CosmTxResponse, color_eyre::Report>::Ok(resp)
        })?;

        let code_id = resp.uploaded_code_id().unwrap();
        resp.log();
        println!("code_id: {code_id}");

        Ok(StoreWasmOutput)
    }
}
