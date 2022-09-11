use std::rc::Rc;

use anyhow::Ok;
use secp256k1::All;
use tokio::runtime::Runtime;

use crate::{sender::Sender, Daemon, DaemonState, NetworkInfo};

pub(crate) mod daemon;
pub(crate) mod mock_chain;

pub fn instantiate_daemon_env(
    network: NetworkInfo<'static>,
) -> anyhow::Result<(Rc<Runtime>, Rc<Sender<All>>, Daemon)> {
    let rt = Rc::new(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?,
    );
    let state = Rc::new(rt.block_on(DaemonState::new(network))?);
    let sender = Rc::new(Sender::new(&state)?);
    let chain = Daemon::new(&sender, &state, &rt)?;
    Ok((rt, sender, chain))
}
