# Integrations

cw-orchestrator aims at allowing developers to interact with different execution environments with the exact same code base and syntax.

In order to do so, it provides a set of traits and interfaces, e.g. for [contracts](../single_contract/index.md) that can be implemented on execution structures. As of writing this documentation, the following execution environments are supported:

- [Node interaction (grpc)](./daemon.md)
- [Osmosis Test Tube](./osmosis-test-tube.md)
- [Cw-Multi-test](./cw-multi-test.md)
- [Fork Testing](./fork-testing.md)

Those environments allow developers to test, deploy, manage, migrate, maintain, without having to switch programming languages, care for types and version compatibilities as they are enforced by the compiler and even change the syntax of the code used.

The [Unit Tests](./unit-tests.md) page shows you how you can unit-test your contracts against actual on-chain data!
