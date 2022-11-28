# Quickstart

In order to use BOOT you need access to the endpoint message types (`InstantiateMsg`,`ExecuteMsg`,...) of the contracts you want to interact with.

If you want to perform on-chain transaction you also need access to the gRPC endpoint of a node. These are most-often available on port 9050.

The following sections detail setting up a library package for the contract interfaces and a binary package for executable scripts.

## Sections
- **[Interfaces](./interfaces.md)**
    * Define interfaces for your contracts.
- **[Environment File](./env-variable.md)**
    * Configure your mnemonics and log settings.
- **[Scripting](./scripting.md)**
    * Write runnable scripts with your interfaces.
