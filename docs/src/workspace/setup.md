# Workspace `cw-orch` setup process

 In all this tutorial, we will use the [`cw-plus`](https://github.com/AbstractSDK/cw-plus) package as an example. This repository is maintained by `CosmWasm` and contains standard contracts. It's also a good workspace structure example.

## Crate setup

If you are using multiple contracts in a single project, we advise you to centralize all the interfaces to your contracts in a single location. To do so, go inside your `packages` folder and create a new `interface` library crate:

```bash
cargo new interface --lib
cd interface
```

Then, going into this crate, we need to add `cw-orch` as a dependency:

```bash
cargo add cw-orch --package interface 
```

`cw-orch` here is not optional, because this `interface` crate is not a contract and will never be included in a contract.

## Individual contract interface

Now inside this crate, you can add the interfaces to all your contracts individually. A sane way to do so is to create a new file (also called module) for each contract. For instance, [the `interface` folder in the `cw-plus` repository](https://github.com/AbstractSDK/cw-plus/tree/main/packages/interface) has the following structure:

```path
.
├── Cargo.toml
└── src
    ├── cw1_subkeys.rs
    ├── cw1_whitelist.rs
    ├── cw20_base.rs
    ├── cw20_ics20.rs
    ├── cw3_fixed_multisig.rs
    ├── cw3_flex_multisig.rs
    ├── cw4_group.rs
    ├── cw4_stake.rs
    └── lib.rs
```

Inside each file, you can define your contract interface. You can find the tutorial for that in the dedicated 