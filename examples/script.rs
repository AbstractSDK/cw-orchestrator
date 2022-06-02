use cw20::Cw20QueryMsg;

use terra_rust_script::{helpers::get_configuration, traits::WasmQuery};

pub async fn script() -> anyhow::Result<()> {
    let (sender, config) = &get_configuration("uusd").await?;

    let cw20_token = scripts::contract_instances::cw_20::CW20::new("cw20", sender, config)?;
    let token_info = cw20_token.query(Cw20QueryMsg::TokenInfo {}).await?;
    print!("{}", token_info);

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = script().await {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}
