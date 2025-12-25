# Installing pyvcloud (Rust)

The Rust toolchain is now the only requirement for building and running `pyvcloud`.

## Prerequisites

- Rust 1.75 or newer. Install via [rustup](https://rustup.rs/) if it is not already present.

## Building the library and CLI

```bash
git clone https://github.com/vmware/pyvcloud.git
cd pyvcloud

# format, build, and test
cargo fmt
cargo build
cargo test
```

The `pyvcloud` binary will be available at `target/debug/pyvcloud` after a successful build. Use `cargo build --release` to generate an optimized binary under `target/release/pyvcloud`.

## Installing from the workspace

You can install the binary into your Cargo bin directory (usually `~/.cargo/bin`) with:

```bash
cargo install --path .
```

After installation, verify the CLI is on your PATH:

```bash
pyvcloud --help
```
