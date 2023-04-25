# boot-cw-plus

Library that hold the most-used interfaces for standard cw-plus contracts and their artifacts.

## Usage

Add this to your `Cargo.toml` and fill in the version of BOOT that you want to use:

```toml
[dependencies]
boot-cw-plus = { git = "https://github.com/AbstractSDK/BOOT", tag = "..."}
```

and then import CwPlus and deploy it to the environment:

```rust
use boot_core::*;
use boot_cw_plus::{CwPlus};

fn setup<Chain: CwEnv>(chain: Chain) {
    ...
    let cw_plus = CwPlus::store_on(chain);
    // get the cw20 token interface
    let cw_20_base: Cw20<Chain> = cw_plus.cw_20_base;
    ...
}
```


## Wasms (v1.0.1)

This package contains the WASM files provided in the github release of cw-plus. You can fetch them yourself using the executable in this library by calling

```bash
cargo run --features "wasms" download_wasms
```
