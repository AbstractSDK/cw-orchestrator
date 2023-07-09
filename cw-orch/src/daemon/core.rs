use crate::{
    daemon::{DaemonAsync, DaemonState},
    state::ChainState,
};

use std::rc::Rc;

impl ChainState for DaemonAsync {
    type Out = Rc<DaemonState>;

    fn state(&self) -> Self::Out {
        self.state.clone()
    }
}
