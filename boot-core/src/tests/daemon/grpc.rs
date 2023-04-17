/*
    This test asserts breaking issues around the GRPC connection
*/
#[cfg(test)]
mod grpc {
    use std::sync::Arc;

    use boot_core::{instantiate_daemon_env, DaemonOptionsBuilder};
    use speculoos::prelude::*;
    use tokio::runtime::Runtime;

    #[test]
    fn no_connection() {
        let runtime = Arc::new(Runtime::new().unwrap());

        let mut network = boot_core::networks::LOCAL_JUNO;
        let grpcs = &vec!["https://127.0.0.1:99999"];
        network.grpc_urls = grpcs;

        let options = DaemonOptionsBuilder::default()
            .network(network)
            .deployment_id("v0.1.0")
            .build()
            .unwrap();

        asserting!("there is no GRPC connection")
            .that(
                &instantiate_daemon_env(&runtime, options)
                    .err()
                    .unwrap()
                    .to_string(),
            )
            .is_equal_to(String::from(
                "Can not connect to any grpc endpoint that was provided.",
            ))
    }

    #[test]
    fn network_grpcs_list_is_empty() {
        let runtime = Arc::new(Runtime::new().unwrap());

        let mut network = boot_core::networks::LOCAL_JUNO;
        let grpcs: &Vec<&str> = &vec![];
        network.grpc_urls = grpcs;

        let options = DaemonOptionsBuilder::default()
            .network(network)
            .deployment_id("v0.1.0")
            .build()
            .unwrap();

        asserting!("GRPC list is empty")
            .that(
                &instantiate_daemon_env(&runtime, options)
                    .err()
                    .unwrap()
                    .to_string(),
            )
            .is_equal_to(String::from("The list of grpc endpoints is empty"))
    }
}
