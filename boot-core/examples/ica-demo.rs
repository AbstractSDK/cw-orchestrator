use boot_core::{
    networks::{osmosis::OSMO_2, JUNO_1},
    *,
};
use cosmwasm_std::{CosmosMsg, Empty};
use simple_ica_controller::msg::{self as controller_msgs, AccountResponse};
use simple_ica_host::msg::{self as host_msgs, ListAccountsResponse};
use tokio::runtime::Runtime;
use std::sync::Arc;

const CRATE_PATH: &str = env!("CARGO_MANIFEST_DIR");
const JUNO_MNEMONIC: &str = "dilemma imitate split detect useful creek cart sort grow essence fish husband seven hollow envelope wedding host dry permit game april present panic move";
const OSMOSIS_MNEMONIC: &str = "settle gas lobster judge silk stem act shoulder pluck waste pistol word comfort require early mouse provide marine butter crowd clock tube move wool";
const JUNO: &str = "juno-1";
const OSMOSIS: &str = "osmosis-2";
#[contract(
    controller_msgs::InstantiateMsg,
    controller_msgs::ExecuteMsg,
    controller_msgs::QueryMsg,
    Empty
)]
struct Controller;

impl<Chain: CwEnv> Controller<Chain> {
    pub fn new(name: &str, chain: Chain) -> Self {
        let mut contract = Contract::new(name, chain);
        contract = contract.with_wasm_path(format!(
            "{CRATE_PATH}/examples/wasms/simple_ica_controller.wasm"
        ));
        Self(contract)
    }
}

#[contract(host_msgs::InstantiateMsg, Empty, host_msgs::QueryMsg, Empty)]
struct Host;
impl<Chain: CwEnv> Host<Chain> {
    pub fn new(name: &str, chain: Chain) -> Self {
        let mut contract = Contract::new(name, chain);
        contract =
            contract.with_wasm_path(format!("{CRATE_PATH}/examples/wasms/simple_ica_host.wasm"));
        Self(contract)
    }
}

// just for uploading
#[contract(Empty, Empty, Empty, Empty)]
struct Cw1;

impl<Chain: CwEnv> Cw1<Chain> {
    pub fn new(name: &str, chain: Chain) -> Self {
        let mut contract = Contract::new(name, chain);
        contract =
            contract.with_wasm_path(format!("{CRATE_PATH}/examples/wasms/cw1_whitelist.wasm"));
        Self(contract)
    }
}

// Requires a running local junod with grpc enabled
pub fn script() -> anyhow::Result<()> {
    let rt: Arc<tokio::runtime::Runtime> = Arc::new(tokio::runtime::Runtime::new().unwrap());

    let interchain = InterchainInfrastructure::new(
        &rt,
        vec![(JUNO_1, JUNO_MNEMONIC), (OSMO_2, OSMOSIS_MNEMONIC)],
    )?;

    let juno = interchain.daemon(JUNO);
    let osmosis = interchain.daemon(OSMOSIS);

    let mut cw1 = Cw1::new("cw1", juno.clone());
    let mut host = Host::new("host", juno.clone());
    let mut controller = Controller::new("controller", osmosis.clone());


    juno.
    // ### SETUP ###
    // deploy_contracts(&mut cw1, &mut host, &mut controller)?;
    // interchain
    //     .hermes
    //     .create_channel(&rt, "connection-0", "simple-ica-v2", &controller, &host);

    // Get account information
    // let res: controller_msgs::ListAccountsResponse =
    //     controller.query(&controller_msgs::QueryMsg::ListAccounts {})?;
    // println!("After channel creation: {:?}", res);
    let remote_accounts: ListAccountsResponse =
        host.query(&host_msgs::QueryMsg::ListAccounts {})?;
    println!("Remote accounts: {:?}", remote_accounts);
    let remote_account = remote_accounts.accounts[1].clone();
    // send some funds to the remote account
    // let res = rt.block_on(juno.sender.bank_send(
    //     &remote_account.account,
    //     vec![cosmwasm_std::coin(100u128, "uosmo")],
    // ))?;
    // println!("Send funds result: {:?}", res);
    let channel = remote_account.channel_id;

    // controller.execute(
    //     &controller_msgs::ExecuteMsg::SendFunds {
    //         ica_channel_id: channel.clone(),
    //         transfer_channel_id: "channel-0".to_string(),
    //     },
    //     Some(&[cosmwasm_std::coin(100u128, "uosmo")]),
    // )?;

    
    query_tx(&rt, &osmosis,"DF7E4A25663D0A4472DFE0546FA561129C9CF36C3E78EEFAB79C00460A9C3711" );
    // let cont_accounts: controller_msgs::ListAccountsResponse = controller.query(&controller_msgs::QueryMsg::ListAccounts {  })?;

    // println!("Controller accounts: {:?}", cont_accounts);

    // controller.execute(
    //     &controller_msgs::ExecuteMsg::SendMsgs {
    //         channel_id: "channel-3".to_string(),
    //         msgs: vec![CosmosMsg::Bank(cosmwasm_std::BankMsg::Burn {
    //             amount: vec![cosmwasm_std::coin(100u128, "ujuno")],
    //         })],
    //         callback_id: None,
    //     },
    //     None,
    // )?;

    // // wait a bit
    // std::thread::sleep(std::time::Duration::from_secs(60));
    // let balance_result: AccountResponse =
    //     controller.query(&controller_msgs::QueryMsg::Account {
    //         channel_id: channel,
    //     })?;
    // println!("Balance result: {:?}", balance_result);
    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = script() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));
        ::std::process::exit(1);
    }
}

fn deploy_contracts(
    cw1: &mut Cw1<Daemon>,
    host: &mut Host<Daemon>,
    controller: &mut Controller<Daemon>,
) -> anyhow::Result<()> {
    cw1.upload()?;
    host.upload()?;
    controller.upload()?;
    host.instantiate(
        &host_msgs::InstantiateMsg {
            cw1_code_id: cw1.code_id()?,
        },
        None,
        None,
    )?;
    controller.instantiate(&controller_msgs::InstantiateMsg {}, None, None)?;
    Ok(())
}

/*

hermes query channels  --chain juno-1
hermes query channels  --chain osmosis-2

hermes query packet pending --chain juno-1 --port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --channel channel-1
hermes query packet pending --chain osmosis-2 --port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --channel channel-1

hermes query packet commitments --chain juno-1 --port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --channel channel-1
hermes query packet commitments --chain osmosis-2 --port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --channel channel-1

hermes query packet pending-sends --chain osmosis-2 --port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --channel channel-1
hermes query packet pending-sends --chain juno-1 --port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --channel channel-1

hermes clear packets --chain juno-1 --port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --channel channel-1
hermes clear packets --chain osmosis-2 --port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --channel channel-1

hermes clear packets --chain osmosis-2 --port transfer --channel channel-0


 */


fn query_tx(rt: &Runtime, chain: &Daemon, hash: &str) {
    let osmo_grpc_channel = chain.sender.as_ref().channel();
    let tx = rt.block_on(DaemonQuerier::find_tx_by_hash(osmo_grpc_channel,hash)).unwrap();
    println!("tx: {:?}", tx);
}