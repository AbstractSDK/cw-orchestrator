# CosmWasm Orch Command Line Interface (CLI)

The CosmWasm Orch CLI is a tool designed to facilitate the development, deployment, and interaction with CosmWasm smart contracts on Cosmos blockchains. It enables developers to create, test, and manage contracts using the interactive CLI and easily deploy them onto supported Cosmos networks.

## Installation

### Prerequisites

- Rust
- OpenSSL
- Access to keyring

### Cargo

```bash
cargo install cw-orch-cli
```

### Add last command to the shell history (Optional)

If Cw Orch CLI ran in interactive mode it's executed command will **not** be appended to your shell history. This means you will not be able to `arrow up` to get the last command and tweak it to your liking.

To solve this you can add the function below to your `~/.bashrc` or similar.
This function wraps the CLI and appends its executed action to your current shell history, enabling you to retrieve it from the history.

```bash
cw-orch-cli() {
  command=$(command cw-orch-cli "$@" | tee /dev/tty | grep 'Your console command' | cut -f2- -d':')
  if [ "$command" != "cw-orch-cli" ]
  then
    history -s cw-orch-cli "$@" # if you still want to be able `arrow up` to the original command
  fi
  history -s $command
}
```

## Usage

The CLI supports two modes of execution: interactive and non-interactive.

### Interactive mode

In interactive mode the CLI will guide you through complex tasks by reducing the initial command's complexity, and ensuring a more intuitive user experience.

The interactive mode will prompt you for new information when needed as you go through the process of creating, testing, and deploying a contract.

Example:

```bash
cw-orch-cli --verbose
```

### Non-interactive mode

You can utilize the non-interactive mode for scripting, automated operations, and tweaking of the interactive mode's commands. Often you'll find yourself using the interactive mode to get the command you need, and then debug it with the non-interactive mode.

Example:

```bash
cw-orch-cli action uni-6 cw query raw juno1czkm9gq96zwwncxusgzruvpuex4wjf4ak7lms6q698938k529q3shmfl90 raw contract_info
```

### Global optional arguments

- `-v` or `--verbose` - enable verbose mode, this will log actions from cw-orch daemon executions that corresponds to your `RUST_LOG` level
- `-s` or `--source-state-file` - source cw-orch state file(`STATE_FILE` [cw-orch env variable]) to use together with address-book entries (address book have higher priority)
- --deployment-id <DEPLOYMENT_ID> - cw-orch state deployment-id, defaults to "default"

[cw-orch env variable]: ../docs/src/contracts/env-variable.md
