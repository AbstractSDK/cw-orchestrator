# Daemon Interchain Environment

This environment allows to interact with actual COSMOS SDK Nodes. Let's see how that work in details: 


## Environment creation

### For scripting

When scripting with `cw-orch-interchain`, developers don't have to create chain `Daemon` objects on their own. They can simply pass chain data to the interchain constructor, and it will create the daemons for them. Like so :

```rust
use cw_orch::prelude::*;
use cw_orch::tokio::runtime::Runtime;
use cw_orch::prelude::networks::{LOCAL_JUNO, LOCAL_OSMO};
use cw_orch_interchain_daemon::{ChannelCreationValidator,DaemonInterchainEnv};

let rt = Runtime::new()?;
let mut interchain = DaemonInterchainEnv::new(rt.handle(), vec![
    (LOCAL_JUNO, None),
    (LOCAL_OSMO, None)
], &ChannelCreationValidator)?;
```

They can access individual `Daemon` objects like so: 
```rust
use cw_orch_interchain_core::InterchainEnv;
let local_juno: Daemon = interchain.chain("testing")?;
let local_osmo: Daemon = interchain.chain("localosmosis")?;
```

where the argument of the `chain` method is the chain id of the chain you are interacting with. Note that this environment can't work with chains that have the same `chain_id`. 

> **NOTE**: Here the `ChannelCreationValidator` struct is a helper that will simply wait for channel creation when it's called in the script. [More information on that channel creation later](#ibc-channel-creation).

You can also add daemons manually to the `interchain` object : 
```rust
let local_migaloo = DaemonBuilder::default()
    .handle(rt.handle())
    .chain(LOCAL_MIGALOO)
    .build()?;
interchain.add_daemons(vec![local_migaloo]);
```

### For testing



It can be used with 2 modes. 

1.  sdfsfd
2.  
3. If needed you can also create you own channel creator. This is fully customizable and if your application already contains a relayer capable of creating channels, it can integrate with `cw-orch-interchain`.

## IBC Channel creation