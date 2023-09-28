# Mock Interchain Environment

This environment allows you to test your IBC application inside your Rust environment without having to create new crates, new packages or even run other programs in the background. This is perfect for iterating through your designs and testing on the go without additional wait time or setup tutorials. 

## Environment creation

You can create your interchain environment using the following simple setup script : 

```rust
use cw_orch::prelude::*;
use cw_orch_interchain::interchain::MockInterchainEnv;

let sender = Addr::unchecked("sender");
let mut interchain = MockInterchainEnv::new(
    vec![("juno-1", &sender), ("osmosis-1", &sender)],
);
```
Behind the scenes, `Mock` objects are created with the specified chain ids. These mock environments can be used on their own to interact with testing environment directly. You can get those objects like so : 
```rust
use cw_orch_interchain::interchain::InterchainEnv;
let local_juno: Daemon = interchain.chain("juno-1")?;
let local_osmo: Daemon = interchain.chain("osmosis-1")?;
```

where the argument of the `chain` method is the chain id of the chain you are interacting with. Note that this environment can't work with chains that have the same `chain_id`. 

You can also add mocks manually to the `interchain` object, after instantiation : 
```rust
let test_migaloo = Mock::new(&sender);
interchain.add_mocks(vec![test_migaloo]);
```

## General Usage

All interchain environments are centered around the `follow_packet` function. In the Mock case, this function is responsible for relaying the packets between the different chains. Using the exact same interface as with other environments, it takes care of all packet relaying procedures.

> **NOTE**: Packets can either go through a successful cycle or timeout. In our current Mock implementation, it's difficult to do it so that packets timeout. We are working on providing tools for developers to test all edge cases and timeout is part of the helpers we want to bring ASAP. 

This function will relay a packets succesfully from the receiving chain back the the source chain. Here is what the full cycle looks like : 

1. <span style="color:purple">⬤</span> On the `source chain`, it queries the packet data associated with the packet channel and sequence.
2. <span style="color:red">⬤</span> On the `destination chain`, it triggers a receive transaction for that packet. 
3. <span style="color:purple">⬤</span> On the `source chain`, it finally triggers an acknowledgement transaction with the data the `destination_chain` returned.


The `wait_ibc` function is very similar except that instead of following a single packet, it follows all packets that are being sent within a transaction. This works in a very similar manner and will never return a timeout transaction. This function is recursive as it will also look for packets inside the receive/ack transactions and also follow their IBC cycle. You can think of this function as going down the rabbit-hole of IBC execution and only returning when all IBC interactions are complete. 

> **NOTE**: most of the methods on the `interchain` variable presented here are [async methods](https://rust-lang.github.io/async-book/). We recommend reading more about async functions at the point. If you're not working with any async functions, the gist here is: 
>    ```rust
>    let runtime = tokio::runtime::Runtime::new()?;
>    runtime.block_on(
>        interchain.wait_ibc(
>            &chain_id,
>            tx_response
>        )
>    )
>    ```

## IBC Channel creation

cw-orchestrator also provides tooling for creating channels between mock environments. Here is how you do it : 
```rust
    use ibc_relayer_types::core::ics24_host::identifier::PortId;
    let rt = tokio::runtime::Runtime::new()?;
    let src_chain = "juno-1".to_string();
    let dst_chain = "juno-1".to_string();
    let port_id = PortId::transfer();
    let (channel, channel_creation_result) = rt.block_on(
        interchain.create_channel(&src_chain, &dst_chain, None, &port_id, &port_id, "ics20-1")
    )?;
```

- The resulting `channel` object allows you to identify the channel that was just created. It can be useful to retrieve the channel identifiers for instance
- The resulting `channel_creation_result` object allows you to identify the different steps of channel creation as well as following all the packets that have been sent during this creation. This is very useful to analyze the effects of the channel creation on external contracts and structures. 