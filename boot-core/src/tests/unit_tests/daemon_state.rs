/*
    DaemonState
*/
#[cfg(test)]
mod general {
    use speculoos::prelude::*;

    use tokio::runtime::Runtime;

    use crate::{
        daemon::state::{
            DaemonState,
            DaemonOptionsBuilder
        },
        networks
    };

    #[test]
    fn daemon_state() {
        let rt = Runtime::new().unwrap();

        let network = networks::PISCO_1;
        let deployment_id = "PISCO_1_TEST_DEPLOY".to_string();

        // build options with DaemonOptionsBuilder
        let options = DaemonOptionsBuilder::default()
            .network(network)
            .deployment_id(deployment_id)
            .build()
            .unwrap();

        let daemon_state = DaemonState::new(options);

        // start the daemon
        let res = rt.block_on(daemon_state).unwrap();

        res.check_file_validity();

        let fp = std::env::var("STATE_FILE").unwrap();
        let b = std::path::Path::new(&fp).exists();

        // actually...
        // we should not be here if it was not created after connecting
        // so it's a bit redundant
        asserting("STATE_FILE was created succesful")
            .that(&b)
            .is_equal_to(&true);
    }
}