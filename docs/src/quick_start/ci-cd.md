# Continuous Integration and Deployment
One of the tools that can improve your developer productivity drastically is setting up pipelines for your contract deployments. 

BOOT does not *currently* add additional support for actions, but an example using the directory structure specified in [interfaces](./interfaces) can be found below:

```yaml
---  
name: Deploy Contracts  
on:  
  # https://docs.github.com/en/actions/reference/events-that-trigger-workflows#workflow_dispatch  
  workflow_dispatch:  
  push:  
    branches: [ 'mainline' ]  
    paths:  
      - 'contracts/src/**/*.rs'  
      - '.github/workflows/deploy.yml'  
  
env:
  VERSION_CONTROL_ADDRESS: juno16enwrxhdtsdk8mkkcaj37fgp37wz0r3err4hxfz52lcdyayexnxs4468mu  
  STATE_FILE: "./daemon_state.json"  
  WASM_DIR: "./target/wasm32-unknown-unknown/release"  
  SCHEMA_DIR: "./schema"  
  
jobs:  
  deploy:  
    runs-on: ubuntu-latest  
    steps:  
      - uses: actions/checkout@v3  
      - uses: actions-rs/toolchain@v1  
        with:  
          toolchain: nightly  
          target: wasm32-unknown-unknown  
          override: true  
      - name: Run cargo wasm  
        uses: actions-rs/cargo@v1  
        with:  
          command: build  
          args: --package counter-app --release --target wasm32-unknown-unknown  
        env:  
          RUSTFLAGS: '-C link-arg=-s'  

      - name: Run deployment script  
        uses: actions-rs/cargo@v1  
        with:  
          command: run  
          args: --package scripts --bin deploy_app  
        env:  
          CHAIN: "juno"  
          DEPLOYMENT: "debugging"  
          NETWORK: "local"  
          RUST_LOG: info  
          WASM_DIR: ${{ env.WASM_DIR }}  
          DAEMON_STATE_PATH: ${{ env.STATE_FILE }}  
  
          VERSION_CONTROL_ADDRESS: ${{ env.VERSION_CONTROL_ADDRESS }}  
          TEST_MNEMONIC: ${{ secrets.TEST_MNEMONIC }}  

      - name: Upload deployment daemon state  
        uses: actions/upload-artifact@v2  
        with:  
          name: deployment.json  
          path: ${{ env.STATE_FILE }}  
      - name: Upload WASM  
        uses: actions/upload-artifact@v2  
        with:  
          # TODO: name env or from cargo  
          name: counter_app.wasm  
          path: ${{ env.WASM_DIR }}/counter_app.wasm
```