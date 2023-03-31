# boot-cw-plus

Library that hold the most-used interfaces for standard cw-plus contracts and their artifacts.

## Usage

add this to your `Cargo.toml`:

```toml
[dependencies]
boot-cw-plus = { git = "" }
```

and then import CwPlus and deploy it to the environment:

```rust

```
```


## Wasms (v1.0.1)

This package contains the wasms files provided in the github release of cw-plus. You can fetch them yourself using the executable in this library by calling

```bash
cargo run --features "wasms" download_wasms
```
