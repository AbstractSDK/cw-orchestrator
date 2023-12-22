# CosmWasm Orch Command Line Interface (CLI)

The CosmWasm Orch CLI is a tool designed to facilitate the development, deployment, and interaction with CosmWasm smart contracts on Cosmos blockchains. It enables developers to create, test, and manage contracts using the interactive CLI and easily deploy them onto supported Cosmos networks.

# Installation

## Prerequisites
It shares same prerequisites as Cw Orch, see [Prerequisites](../INSTALL.md#prerequisites)

## Install the Cosmwasm Orch CLI
```bash
cargo install cw-orch-cli
```

## Add latest command to the shell history (Optionally)
If Cw Orch CLI ran in interactive mode shell history will only save the command it was originally executed with.
For more enjoyable user experience you can add this function to ~/.bashrc to append the last executed command to the current session history:
```bash
cw-orch-cli() {
	command=$(command cw-orch-cli "$@" | tee /dev/tty | grep 'Your console command' | cut -f2 -d':')
	history -s $command
}
```

# Usage

## Interactive mode
Interactive mode simplifies complex tasks, reduces command complexity, and ensures a more intuitive user experience through real-time interaction. Which makes it preferred way of execution for users.

## Non-interactive mode
Utilize the non-interactive mode for scripted tasks, automated operations, and efficient execution without manual interaction within the CW-Orchestrator CLI.

Example: 
```bash
cw-orch-cli action uni-6 cw query raw juno1czkm9gq96zwwncxusgzruvpuex4wjf4ak7lms6q698938k529q3shmfl90 contract_info
```
