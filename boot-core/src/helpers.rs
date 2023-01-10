use std::env;

pub fn get_env_vars() -> (String, String, String, bool) {
    let propose_on_multisig =
        env::var("PROPOSE_ON_MULTISIG").unwrap_or_else(|_| "false".to_string());
    let store_path = env::var("STORE").expect("STORE is not set");
    let chain = env::var("CHAIN").expect("CHAIN is not set");
    let deployment = env::var("DEPLOYMENT").expect("DEPLOYMENT is not set");

    (
        store_path,
        chain,
        deployment,
        propose_on_multisig.parse::<bool>().unwrap(),
    )
}
