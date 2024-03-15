# Key management

To sign transactions, you need to have a stored key in the keyring. This is currently the only way to sign transactions, feel free to request other signing methods.

## Safety
The keys are kept in an underlying platform-specific secure store(keyring) as seeds. To support different derivation paths we can't save it as key pair

## Features

#### Add key

Add key command saves provided or generated seed to the keyring
- Generate new random seed : `cw-orch-cli key add [NAME] new`
- Recover from seed phrase: `cw-orch-cli key add [NAME] from-seed`
  - This command will give you prompt
#### Show seed of saved key

Show seed command loads saved seed phrase from keyring and outputs it
- Shows seed phrase of the key: `cw-orch-cli key show [NAME]`

#### Show address

Show address command generates public address for this key on chosen network
- Show address: `cw-orch-cli key show-address [NAME] [CHAIN_ID]`
#### Remove key

Remove key command deletes entry of provided key-id from the keyring
- Removes key: `cw-orch-cli key remove [NAME]`