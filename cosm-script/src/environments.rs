use std::{cell::RefCell, rc::Rc};

use anyhow::Ok;
use cosmwasm_std::Addr;
use cw_multi_test::{App, BasicApp};
use secp256k1::All;
use tokio::runtime::Runtime;

use crate::{
    sender::Sender, state::StateInterface, Daemon, DaemonState, Mock, MockState, NetworkInfo,
};

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

pub fn instantiate_default_mock_env(
    sender: &Addr,
) -> anyhow::Result<(Rc<RefCell<MockState>>, Rc<RefCell<App>>, Mock<MockState>)> {
    let mock_state = Rc::new(RefCell::new(MockState::new()));
    let mock_app = Rc::new(RefCell::new(BasicApp::new(|_, _, _| {})));
    let mock_chain = Mock::new(&sender, &mock_state, &mock_app)?;
    Ok((mock_state, mock_app, mock_chain))
}
pub fn instantiate_custom_mock_env<S: StateInterface>(
    sender: &Addr,
    custom_state: S,
) -> anyhow::Result<(Rc<RefCell<S>>, Rc<RefCell<App>>, Mock<S>)> {
    let mock_state = Rc::new(RefCell::new(custom_state));
    let mock_app = Rc::new(RefCell::new(BasicApp::new(|_, _, _| {})));
    let mock_chain = Mock::new(&sender, &mock_state, &mock_app)?;
    Ok((mock_state, mock_app, mock_chain))
}
