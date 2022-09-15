pub fn my_script() -> anyhow::Result<()> {
    // Write your script here
    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = my_script() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        log::error!("Ensure your environment variables are set!");
        ::std::process::exit(1);
    }
}
