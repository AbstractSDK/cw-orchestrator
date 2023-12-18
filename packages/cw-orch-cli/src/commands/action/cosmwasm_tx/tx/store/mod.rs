use color_eyre::eyre::Context;
use cw_orch::{
    prelude::{DaemonAsync, IndexResponse},
    tokio::runtime::Runtime,
};

use crate::commands::action::CosmosContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = StoreWasmOutput)]
/// Execute contract method
pub struct StoreContractCommands {
    /// Input path to the wasm
    wasm_path: crate::types::PathBuf,
    /// Signer id
    // TODO: should be possible to sign it from the seed phrase
    signer: String,
}

pub struct StoreWasmOutput;

impl StoreWasmOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<StoreContractCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let seed = crate::common::seed_phrase_for_id(&scope.signer)?;
        let wasm_byte_code = std::fs::read(&scope.wasm_path).wrap_err(format!(
            "Failed to open or read the file: {}",
            scope.wasm_path.0.display()
        ))?;

        let rt = Runtime::new()?;
        rt.block_on(async {
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
            let result = daemon.sender.commit_tx(vec![exec_msg], None).await?;

            println!("Uploaded: {:?}", result.txhash);

            let code_id = result.uploaded_code_id().unwrap();

            println!("code_id: {code_id}");

            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(StoreWasmOutput)
    }
}
