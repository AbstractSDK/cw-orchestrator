# Interchain Execution Environment Integration

When working with interchain applications, you will run into issues testing your application or even executing your application on real chains. The issues might me the following:

## Packet relaying

When interacting with actual chains, you might not be able to make sure that packets have been relayed correctly and often times, developers rely on `sleep`, timeout, application specific events or data availability to follow their interchain executions. And this is a mess because this might not be reliable, or un-reusable and application specific code. With `cw-orch-interchain` we provide tooling to follow and assert your packet execution. This allows developers to create robust deployment, testing and maintenance code. [Try it out today](./daemon.md).

## Testing

When testing interchain applications, you run into the following issues: 

### No IBC testing framework available

The most used library for cosmwasm testing is <a target="_blank", href="https://github.com/CosmWasm/cw-multi-test/">Cw-Multi-Test</a>. However, this library doesn't support IBC. `cw-orch` provides interchain capabilities by default. [Try it out today](./mock.md).

### IBC testing with Custom Modules

Some Chains have very interesting Custom Modules that work with IBC (IBC Queries on Neutron for instance). However, it's not possible to test those custom modules with local in-code testing frameworks[^1]. `cw-orchestrator` allows using `Starship`, a local testing framework that spins-up a mini cosmos ecosystem of chains and relayers locally. The used chains are clones of actual chain binaries  So for instance, starship can spin up a neutron chain locally and test the logic directly against actual on-chain go binaries. [Try it out today](./starship.md).


[^1]: Because that would require duplicating the go logic inside Rust code which is very time-consuming to do and maintain.