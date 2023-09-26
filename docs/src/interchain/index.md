# Interchain Capabilities

Because of its asynchronous and decentralized nature, the Inter-Blockchain communication protocol makes developing and debugging applications more difficult than simple blockchain transactions. 
Cw-orch simplifies those tasks by providing developers tools, full testing environments and standard interfaces to interact with applications that leverage IBC capabilities. 

Here are a few examples of what cw-orch allows: 

## Interchain packet following
Using some simple tools, one can follow the execution of IBC packets through their whole lifetime (Receive, Acknowledge or Timeout). 
This is mostly useful for packet analysis of certain channels, ports or connections. 

```rust
let packet_lifetime: SimpleIbcPacketAnalysis<Daemon> = interchain.follow_packet(
    "juno",
    "transfer",
    "channel-16",
    "akash",
    3u64.into()
).await?;
```

## Interchain transaction waiting
To further simplify the developer experience and provide simple syntax, cw-orch allows developers to await execution of transactions that create IBC packets. 
This is mostly useful when interacting with existing contracts in a script to simplify the execution pipeline.

```rust
// We order an ICA controller to burn some funds on juno from akash
// Upon submitting this transaction successfully, a packet will be sent from akash to juno to trigger the ICA action
let transaction_response = controller.send_msgs("channel-16", vec![
        CosmosMsg::Bank(cosmwasm_std::BankMsg::Burn {
            amount: vec![cosmwasm_std::coin(100u128, "ujuno")],
        })
    ])?;

// This function won't return before the packet is relayed successfully or times-out. 
let packet_lifetime: IbcTxAnalysis<Daemon> = interchain.wait_ibc(
    "akash",
    transaction_response
).await?;

// You can analyze the packet lifetime
match packet_lifetime.packets[0].outcome{
    IbcPacketOutcome::Success(_) => {},
    _ => panic!("Expected packet to be successfully transmitted")
};

// You can safely continue with the rest of your application, the packet has been successfully relayed
```

## Interchain application testing
Cw-orch allows developers to test their IBC applications and smart-contracts using a common interface. As we know that setting an IBC testing environment is heavy on resources and can be time-consuming, we provide 2 testing environments that will help them streamline their development process : 

### [Rust-only](./integrations/mock.md)

The `MockInterchainEnv` object allows developers to test their application without leaving Rust and without compromising on test run-speed. 

Built on top of cw-multi-test, this environment replicates the actual on-chain IBC module (channel creation as well as packet). This allows you to test any IBC application that leverages Smart-Contract or Bank-module IBC packets. It is really powerful and **doesn't** rely on **ANY** external tools to work. No node setup, no relayer setup, no cluster setup, everything runs inside your crate. Visit the dedicated [Mock Interchain Env](./integrations/mock.md) page for more details and code snippets.


### [Cosmos SDK Node](./integrations/starship.md)

The `Starship` object allows developers to test their application against the actual binaries of running chains. If you want to run your application with specific logic, custom messages or modules, this is the preferred way. This is the IBC version of the local chains that you can run locally. It can also spin up relayers and set up connections between your local chains automatically. 

Visit the dedicated [Starship](./integrations/starship.md) page for more details and code snippets.