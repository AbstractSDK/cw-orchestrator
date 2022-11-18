use std::env;

pub fn get_env_vars() -> (String, String, String, bool) {
    let propose_on_multisig =
        env::var("PROPOSE_ON_MULTISIG").unwrap_or_else(|_| "false".to_string());
    let store_path = env::var("STORE").unwrap();
    let chain = env::var("CHAIN").unwrap();
    let deployment = env::var("DEPLOYMENT").unwrap();

    (
        store_path,
        chain,
        deployment,
        propose_on_multisig.parse::<bool>().unwrap(),
    )
}
