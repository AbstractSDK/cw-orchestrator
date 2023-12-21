## Cw Orch CLI

Cw Orch CLI is a command line utility for working with cosmos blockhains

## Add latest command to the history
You can add this function to ~/.bashrc to append the last executed command to the current session history:
```bash
cw-orch-cli() {
	command=$(command cw-orch-cli "$@" | tee /dev/tty | grep 'Your console command' | cut -f2 -d':')
	history -s $command
}
```