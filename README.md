# Rust template using the Peripheral Access Crate (PAC) for the TM4C123G Controller

This is a template for an embedded Rust application that utilizes the
[ZM4C123x PAC](https://crates.io/crates/tm4c123x) crate for registerlevel programming in Rust.

### Required Software
- [Rustup](https://www.rust-lang.org/tools/install)*: Rust toolchain manager
- OpenOCD for debugging
- arm-none-eabi-gdb for debugging (Can be installed as part of the [GNU Arm Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads))
- [Visual Studio Code](https://code.visualstudio.com/)

\* Rustup is a program to install and manage Rust toolchains for diefferent platforms. Depending on your OS, it may also be possible to install the Rust compiler rustc or the build system cargo without rustup. If this is installed in parallel to rustup, it may cause problems.

Also make sure to add all the required tools to the PATH.

### Setup
Install the thumbv7em-none-eabihf target:
```
rustup target add thumbv7em-none-eabihf
```
Install cargo-generate:
```
cargo install cargo-generate
```
Then use ```cargo generate --git``` with the link to this template repository.
