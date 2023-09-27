# Interchain Quickstart

## General Example

### Creating the environment

In order to interact with your environment using IBC capabilities, you first need to create an interchain structure.
In this guide, we will create a mock environment for local testing. [↓Click here, if you want to interact with actual nodes](#with-actual-cosmos-sdk-nodes).


With mock chains, you can create a mock environment simply by specifying chains ids and sender addresses.
For this guide, we will create 2 chains, `juno` and `osmosis`, with the same address as sender:  
```rust
 let sender = Addr::unchecked("sender_for_all_chains");
 let interchain = MockInterchainEnv::new(vec![("juno", &sender), ("osmosis", &sender)]);
```

### Interacting with the environment

Now, we will work with interchain accounts (ICA). There is a [simple implementation of the ICA protocol on Github](https://github.com/confio/cw-ibc-demo), and we will use that application with a few simplifications for brevity.

In this protocol, we have 2 smart-contracts that are able to create a connection between them. 
The `client` will send IBC messages to the `host` that in turn will execute the messages on its chain. 
Let's first create the contracts :
```rust
let juno = interchain.chain("juno")?;
let osmosis = interchain.chain("osmosis")?;

let client = Client::new("test:client", juno.clone());
let host = Host::new("test:host", osmosis.clone());

client.upload()?;
host.upload()?;
client.instantiate(&Empty{}, None, None)?;
host.instantiate(&Empty{}, None, None)?;

```
The `Client` and `Host` structures here are [cw-orch Contracts](../single_contract/interfaces.md) with registered ibc endpoints. 

<details>
  <summary><strong>Client contract definition</strong> (Click to get the full code)</summary>

```rust
#[interface(
    simple_ica_controller::msg::InstantiateMsg,
    simple_ica_controller::msg::ExecuteMsg,
    simple_ica_controller::msg::QueryMsg,
    Empty
)]
struct Client;

impl<Chain: CwEnv> Uploadable for Client<Chain> {
    // No wasm needed for this example
    // You would need to get the contract wasm to be able to interact with actual Cosmos SDK nodes
    fn wasm(&self) -> WasmPath {
        let wasm_path = format!("No wasm");
        WasmPath::new(wasm_path).unwrap()
    }
    // Return a CosmWasm contract wrapper with IBC capabilities
    fn wrapper(&self) -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                simple_ica_controller::contract::execute,
                simple_ica_controller::contract::instantiate,
                simple_ica_controller::contract::query,
            )
            .with_ibc(
                simple_ica_controller::ibc::ibc_channel_open,
                simple_ica_controller::ibc::ibc_channel_connect,
                simple_ica_controller::ibc::ibc_channel_close,
                simple_ica_controller::ibc::ibc_packet_receive,
                simple_ica_controller::ibc::ibc_packet_ack,
                simple_ica_controller::ibc::ibc_packet_timeout,
            ),
        )
    }
}
```  
</details>

<details>
  <summary><strong>Host contract definition</strong> (Click to get the full code)</summary>

```rust
// This is used because the simple_ica_host contract doesn't have an execute endpoint defined 
pub fn host_execute(_: DepsMut, _: Env, _: MessageInfo, _: Empty) -> StdResult<Response> {
    Err(StdError::generic_err("Execute not implemented for host"))
}

#[interface(
    simple_ica_host::msg::InstantiateMsg,
    Empty,
    simple_ica_host::msg::QueryMsg,
    Empty
)]
struct Host;

impl<Chain: CwEnv> Uploadable for Host<Chain> {
    // No wasm needed for this example
    // You would need to get the contract wasm to be able to interact with actual Cosmos SDK nodes
    fn wasm(&self) -> WasmPath {
        let wasm_path = format!("No wasm");
        WasmPath::new(wasm_path).unwrap()
    }
    // Return a CosmWasm contract wrapper with IBC capabilities
    fn wrapper(&self) -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                host_execute,
                simple_ica_host::contract::instantiate,
                simple_ica_host::contract::query,
            )
            .with_reply(simple_ica_host::contract::reply)
            .with_ibc(
                simple_ica_host::contract::ibc_channel_open,
                simple_ica_host::contract::ibc_channel_connect,
                simple_ica_host::contract::ibc_channel_close,
                simple_ica_host::contract::ibc_packet_receive,
                simple_ica_host::contract::ibc_packet_ack,
                simple_ica_host::contract::ibc_packet_timeout,
            ),
        )
    }
}
```  
</details>

Then, we can create an IBC channel between the two contracts : 

```rust
let channel_receipt = interchain.create_contract_channel(&client, &host, None, "simple-ica-v2").await?;

// After channel creation is complete, we get the channel id, which is necessary for ICA remote execution
let juno_channel = channel.0.get_chain("juno")?.channel.unwrap();
```

This step will also await until all the packets sent during channel creation are relayed. In the case of the ICA contracts, a [`{"who_am_i":{}}`](https://github.com/confio/cw-ibc-demo/blob/main/contracts/simple-ica-controller/src/ibc.rs#L54) packet is sent out right after channel creation and allows to identify the calling chain. 


Finally, the two contracts can interact like so :
```rust
/// This broadcasts a transaction on the client
/// It sends an IBC packet to the host
let tx_response = client.send_msgs(
    juno_channel.to_string(), 
    vec![CosmosMsg::Bank(cosmwasm_std::BankMsg::Burn {
            amount: vec![cosmwasm_std::coin(100u128, "uosmo")],
    })],
    None
)?;
```

Now, we need to wait for the IBC execution to take place and the relayers to relay the packets. This is done through : 
```rust
let packet_lifetime = interchain.wait_ibc("juno", tx_response).await?;
```

After that step, we make sure that the packets were relayed correctly
```rust 
// For testing a successful outcome of the first packet sent out in the tx, you can use : 
if let IbcPacketOutcome::Success{
    ack,
    ..
} = packet_lifetime.packets[0].outcome{
    if let IbcPacketAckDecode::Success(_) = ack{
        /// Packet has been successfully acknowledged and decoded, the transaction has gone through correctly
    }
    /// Else, there was a decode error (maybe you are using the wrong acknowledgement format)
}else{
    /// Else the packet timed-out, you may have a relayer error or something is wrong in your application
};

// OR you can use a helper, that will only error if one of the packets being relayed failed
assert_packets_success_decode(packet_lifetime)?;
```

If it was relayed correctly, we can proceed with our application.


With this simple guide, you should be able to test and debug your IBC application in no time. 
[Learn more about the implementation and details of the IBC-enabled local testing environment](./integrations/mock.md).


## With actual Cosmos SDK Nodes

You can also create an interchain environment that interacts with actual running chains. Keep in mind in that case that this type of environment doesn't allow channel creation. This step will have to be done manually with external tooling. If you're looking to test your application in a full local test setup, please turn to [↓Starship](#with-starship)


```rust
use cw_orch::prelude::*;
use cw_orch::tokio;

// This is used to deal with async functions
let rt = tokio::runtime::Runtime::new().unwrap();

// We create the daemons ourselves to interact with actual running chains (testnet here)
let juno = Daemon::builder()
        .chain(cw_orch::daemon::networks::UNI_6)
        .handle(rt.handle())
        .build()
        .unwrap(); 

// We create the daemons ourselves to interact with actual running chains (testnet here)
let osmosis = Daemon::builder()
        .chain(cw_orch::daemon::networks::OSMO_5)
        .handle(rt.handle())
        .build()
        .unwrap();

// This will allow us to follow packets execution between juno and osmosis
let interchain = DaemonInterchainEnv::from_daemons(
    vec![juno, osmosis],
    &ChannelCreationValidator,
);

// In case one wants to analyze packets between more chains, you just need to add them to the interchain object
```

With this setup, you can now resume this quick-start guide from [↑Interacting with the environment](#interacting-with-the-environment).

You can also [learn more about the interchain daemon implementation](./integrations/daemon.md).

## With Starship

You can also create you interchain environment using starship, which allows you to test your application against actual nodes and relayers. This time, an additional setup is necessary. 
Check out [the official Starship Getting Started guide](https://starship.cosmology.tech/) for more details. 

Once starship is setup and all the ports forwarded, assuming that starship was run locally, you can execute the following : 

```rust
use cw_orch::prelude::*;
use cw_orch::tokio;

// This is used to deal with async functions
let rt = tokio::runtime::Runtime::new().unwrap();

// This is the starship adapter
let starship = Starship::new(rt.handle().to_owned(), None).unwrap();

// You have now an interchain environment that you can use in your application
let interchain = starship.interchain_env();
```

This snippet will identify the local Starship setup and initialize all helpers and information needed for interaction using cw-orch. 
With this setup, you can now resume this quick-start guide from [↑Interacting with the environment](#interacting-with-the-environment)

You can also [learn more about the interchain daemon implementation](./integrations/daemon.md).
