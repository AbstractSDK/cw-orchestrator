# Wasm Compilation

`cw-orch` was designed to help you test, deply, script and maintaint your application. None of its features include in-contract operations. cw-orch interfaces, macros and helpers can't be used inside a wasm smart contract.

In case you use one of the cw-orch features inside you smart-contract, the compilation will send an error anyway, so that we are **SURE** you are not using un-intended features inside your contract.

## Importing cw-orch

Importing `cw-orch` as a dependency in your smart contract is not a problem. We have built this library such that every feature that cw-orch imports, uses or exports is not included when compiled for Wasm. This means that you are safe to use any cw-orch feature when developing your application, creating interfaces, exporting features, because none of it will land in your contract.

In order to make sure you don't encounter wasm compilation errors, you should follow the guidelines outlined in the next section.

## Guidelines

### Imports

Import cw-orch without a worry, this won't include unnecessary dependencies and bloat your smart-contract. Be careful of the features you import `cw-orch` with because they might increase your compile time (especially `daemon` and `osmosis-test-tube`). 

### Interface

The interface macro itself compiles to Wasm to empty traits. So this macro can be used anywhere in your contract. This **IS** smart-contract safe: 

```rust,ignore
{{#include ../../../contracts/counter/src/interface.rs:interface_macro}}
```

However, the `Uploadable` traits implementation **IS NOT** safe for smart contracts and needs to import namely `cw-multi-test` elements that we don't remove from WASM compilation. The following code needs to be flagged to not be compiled inside Wasm contracts:

```rust,ignore
#[cfg(not(target_arch = "wasm32"))]
{{#include ../../../contracts/counter/src/interface.rs:uploadable_impl}}
```

### Entry Points

The entry points are easy to work with as they compile to empty traits inside Wasm. So you can define them, import and export them in your contract without having to care about compilation targets. Furthermore, those traits are optimized out when getting your contract ready to upload on a chain. The syntax use in the 2 following examples is WASM safe:

```rust,ignore
{{#include ../../../contracts/counter/src/msg.rs:exec_msg}}
```

```rust,ignore
{{#include ../../../contracts/counter/src/lib.rs:fn_re_export}}
```
