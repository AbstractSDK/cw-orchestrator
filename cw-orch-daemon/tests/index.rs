mod common;
#[cfg(feature = "node-tests")]
mod tests {

    use cw_orch_core::environment::TxHandler;
    use cw_orch_daemon::Daemon;

    #[test]
    #[serial_test::serial]
    fn mnemonic_index() -> anyhow::Result<()> {
        use cw_orch_networks::networks;

        let runtime = tokio::runtime::Runtime::new().unwrap();

        let daemon = Daemon::builder()
            .chain(networks::LOCAL_JUNO)
            .handle(runtime.handle())
            .build()
            .unwrap();

        let indexed_daemon = Daemon::builder()
            .chain(networks::LOCAL_JUNO)
            .handle(runtime.handle())
            .with_index(56)
            .build()
            .unwrap();

        assert_ne!(
            daemon.sender().to_string(),
            indexed_daemon.sender().to_string()
        );

        Ok(())
    }
}
