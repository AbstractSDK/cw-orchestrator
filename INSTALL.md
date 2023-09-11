
# Installation

Simply run the following command to add cw-orchestrator to your rust crate : 
```bash
cargo add cw-orch
```

Make sure all the [prerequisites](#prerequisites) are met before building packages with cw-orch.

# Prerequisites

Cw-orch relies on external libraries that need additional tools installed on your machine : 
- Rust
- OpenSSL
- Gcc + Clang (optional, only for osmosis-test-tube)
- Go (optional, only for osmosis-test-tube)

After installing the Prerequisites, 

## Ubuntu

### Fast way

  - Install the [go compiler](https://go.dev/doc/install) for osmosis-test-tube.

  - Other installs in one command : 
    ```bash
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; \
        sudo apt install build-essential \
        pkg-config \
        libssl-dev \
        clang
    ```

### Detailed way

Install the following packages : 

1. [The rust toolchain](https://www.rust-lang.org/tools/install)
2. The gcc compiler.
    ```bash
        sudo apt install build-essential
    ``` 
3. The pkg-config library 
    ```bash
        sudo apt install pkg-config
    ``` 
4. The open-ssl development library
    ```bash
        sudo apt install libssl-dev
    ``` 
5. The [go compiler](https://go.dev/doc/install) for osmosis-test-tube
6. The clang library for osmosis test tube
    ```bash
        sudo apt install clang
    ``` 

## Arch Linux

### Fast way

  - You can install all prerequisites in one command : 
    ```bash
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; \
        sudo pacman -Sy gcc \
        pkgconf \
        openssl \
        go \
        clang 
    ```


### Detailed way
Install the following packages : 
1. [The rust toolchain](https://www.rust-lang.org/tools/install)
2. The gcc compiler.
    ```bash
        sudo pacman -Sy gcc
    ``` 
3. The pkg-config library 
    ```bash
        sudo pacman -Sy pkgconf
    ``` 
4. The open-ssl development library
    ```bash
        sudo pacman -Sy openssl
    ``` 
5. The go compiler for osmosis-test-tube
    ```bash
        sudo pacman -Sy go
    ```
6. The clang library for osmosis test tube
    ```bash
        sudo pacman -Sy clang
    ``` 
