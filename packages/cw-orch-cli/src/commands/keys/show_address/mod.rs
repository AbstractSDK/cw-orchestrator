use cw_orch::{
    prelude::{networks::parse_network, Daemon, TxHandler},
    tokio::runtime::Runtime,
};

use crate::common::seed_phrase_for_id;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = ShowAddressOutput)]
pub struct ShowAddressCommand {
    /// Id of the key
    name: String,
    chain_id: String,
}

pub struct ShowAddressOutput;

impl ShowAddressOutput {
    fn from_previous_context(
        _previous_context: (),
        scope:&<ShowAddressCommand as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let mnemonic = seed_phrase_for_id(&scope.name)?;
        let chain = parse_network(&scope.chain_id);

        let rt = Runtime::new()?;
        let daemon = Daemon::builder()
            .handle(rt.handle())
            .chain(chain)
            .mnemonic(mnemonic)
            .build()?;
        let address = daemon.sender();
        println!("Your address: {address}");
        Ok(ShowAddressOutput)
    }
}
