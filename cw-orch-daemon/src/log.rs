use crate::env::DaemonEnvVars;

use crate::DaemonError;

use std::sync::Once;

static LOGS_DISABLED: Once = Once::new();

// Prints a warning if log is disabled for the application
pub fn print_if_log_disabled() -> Result<(), DaemonError> {
    LOGS_DISABLED.call_once(|| {
        // Here we check for logging capabilities.
        if !log::log_enabled!(log::Level::Info) && DaemonEnvVars::logs_message() {
            println!(
                "Warning: It seems like you haven't enabled logs. In order to do so, you have to :
                - use `env_logger::init()` at the start of your script.
                - Set the env variable `RUST_LOG=info` for standard logs. See https://docs.rs/env_logger/latest/env_logger/"
            )
        }
    });
    Ok(())
}
