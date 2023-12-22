use cw_orch::daemon::CosmTxResponse;

pub trait LogOutput {
    fn log(&self);
}

impl LogOutput for CosmTxResponse {
    fn log(&self) {
        println!("Transaction hash: {}", self.txhash);
    }
}
