## TODO (+ not tested, snippets should be included in the code directly)

# Starship integration

Starship is a development environment that allows developers to spin up mini cosmos chain ecosystems and especially allows developers to write and execute IBC integration tests. 

## Quickstart

Before using starship, you need to run it (locally or remotely). Starship provides a [nice tutorial](https://starship.cosmology.tech/) for installing/running and integrating with the framework.

When a starship instance is running, you can use it directly with the rust integration cw-orchestrator provides

```rust,ignore
    let rt = Runtime::new().unwrap();

    let starship = Starship::new(rt.handle().to_owned(), None)?;
```

This snippet connects to a local starship deployment. To get the [daemon](./daemon.md) instance for a specific chain, you can use: 
```rust,ignore
    let juno = starship.daemon("juno-1")?;
```

This daemon instance can be used just like you would with any instance, nothing is added to those instances.

## Creating an IBC channel

The starship object allows you to create a channel between 2 chains very easily : 

```rust,ignore
    starship.client().create_channel(
        "juno-1", 
        "osmosis-1", 
        "transfer",
        "transfer", 
        "ics20-1"
    ).await?;
```

However, if you send additional IBC packets inside this channel creation transaction, you may have to wait a little more until all IBC packets are resolved (this is the case for the ICA WhoAmI procedure). 
In order to create a channel and make sure that the channel creation goes until the end before going to your next occupation, you can leverage the [interchain environment object](../interchain/index.md ) :


```rust,ignore
    let interchain = starship.interchain_env();

    let juno = interchain.daemon("juno-1")?;
    let osmosis = interchain.daemon("osmosis-1")?;

    let interchain_channel = 
        InterchainChannelBuilder::default()
            .from_daemons(juno, osmosis)
            .port_a("transfer")
            .port_b("transfer")
            .create_channel(starship.client(), "ics20-1"),
            .await?;
``` 


## Queries
