mod common;
#[cfg(feature = "node-tests")]
mod tests {

    use cw_orch_core::environment::TxHandler;
    use cw_orch_daemon::{senders::CosmosOptions, Daemon};

    #[test]
    #[serial_test::serial]
    fn mnemonic_index() -> anyhow::Result<()> {
        use cw_orch_networks::networks;

        let daemon = Daemon::builder(networks::LOCAL_JUNO)
            .is_test(true)
            .build()
            .unwrap();

        let daemon_sender = daemon.sender_addr().to_string();
        let indexed_daemon: Daemon = daemon
            .rebuild()
            .build_sender(CosmosOptions::default().hd_index(56))
            .unwrap();

        assert_ne!(daemon_sender, indexed_daemon.sender_addr().to_string());

        Ok(())
    }
}
