# Installation

Add the crate to a Rust project with Cargo:

```toml
[dependencies]
pyvcloud = { path = "../pyvcloud" }
```

For local development in this repository, install a recent Rust toolchain and run:

```shell
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```
