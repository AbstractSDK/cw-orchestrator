use cosm_script::{helpers::get_configuration, traits::*};

use cw_plus_script::CW20;

// Requires a running local junod with grpc enabled

pub async fn script() -> anyhow::Result<()> {
    let (config, sender) = &get_configuration().await?;

    let cw20_token = CW20::new("cw20", &sender, config)?;

    // cw20_token.upload("examples/cw20_base.wasm").await?;

    cw20_token
        .create_new(sender.pub_addr_str()?, 642406u128)
        .await?;
    print!("{:?}", cw20_token.instance().get_address()?);

    cw20_token
        .exec(
            &cw20::Cw20ExecuteMsg::Burn {
                amount: 700u128.into(),
            },
            None,
        )
        .await?;
    let token_info = cw20_token
        .query::<cw20::TokenInfoResponse>(cw20_base::msg::QueryMsg::TokenInfo {})
        .await?;
    print!("{:?}", token_info);

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
