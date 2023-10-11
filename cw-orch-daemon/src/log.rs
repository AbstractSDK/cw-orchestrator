// Prints a warning if log is disabled for the application
pub fn print_if_disabled_log() {
    // Here we check for logging capabilites. We want to help the users as much as possible

    if !log::log_enabled!(log::Level::Info) {
        println!(
            "It seems like you haven't enabled logs. In order to do so, you have to : 
    - use `env_logger::init()` inside your script
    - use the env variable `RUST_LOG=INFO` for standard logs or `RUST_LOG=DEBUG` for more details"
        )
    }
}
