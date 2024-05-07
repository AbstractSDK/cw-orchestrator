mod common;
#[cfg(feature = "node-tests")]
mod tests {

    use cw_orch_core::environment::TxHandler;
    use cw_orch_daemon::Daemon;

    #[test]
    #[serial_test::serial]
    fn mnemonic_index() -> anyhow::Result<()> {
        use cw_orch_networks::networks;

        let daemon = Daemon::builder()
            .chain(networks::LOCAL_JUNO)
            .build()
            .unwrap();

        let daemon_sender = daemon.sender().to_string();
        // TODO: Should be using rebuild here, see ORC-127
        drop(daemon);
        let indexed_daemon = Daemon::builder()
            .chain(networks::LOCAL_JUNO)
            .hd_index(56)
            .build()
            .unwrap();

        assert_ne!(daemon_sender, indexed_daemon.sender().to_string());

        Ok(())
    }
}
