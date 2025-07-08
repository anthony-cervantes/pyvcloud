## pyvcloud

[![License](https://img.shields.io/pypi/l/pyvcloud.svg)](https://pypi.python.org/pypi/pyvcloud) [![Stable Version](https://img.shields.io/pypi/v/pyvcloud.svg)](https://pypi.python.org/pypi/pyvcloud) [![Build Status](https://img.shields.io/travis/vmware/pyvcloud.svg?style=flat)](https://travis-ci.org/vmware/pyvcloud/)

`pyvcloud` started as a Python SDK for VMware vCloud Director. The project is
now migrating to a Rust implementation for better safety and performance.

Supported API versions are 29.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0.

## Building

Development of the Rust version uses the standard cargo workflow. Ensure a
recent Rust toolchain is installed and run the following to verify the build:

```shell
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```


## Notes

Please note that this project is under development and the interfaces might change over time.

`pyvcloud` is used by [vcd-cli](https://vmware.github.io/vcd-cli), the Command Line Interface for VMware vCloud Director. It requires Python 3.6 or higher.

Previous versions and deprecated code can be found in this repository under [tag 18.2.2](https://github.com/vmware/pyvcloud/tree/18.2.2).

## Migration to Rust

This Python implementation of `pyvcloud` is deprecated. Development is moving toward a Rust-based library that follows SOLID and DRY principles.
The new Rust crate lives under `pyvcloud-rs` and currently provides a small `Client` struct and helper utilities, such as `extract_id` for parsing URNs.
See [RUST_MIGRATION_CHECKLIST.md](RUST_MIGRATION_CHECKLIST.md) for an overview of the migration plan and progress.

## Contributing

The `pyvcloud` project team welcomes contributions from the community. Before you start working with `pyvcloud`, please read our [Developer Certificate of Origin](https://cla.vmware.com/dco). All contributions to this repository must be signed as described on that page. Your signature certifies that you wrote the patch or have the right to pass it on as an open-source patch. For more detailed information, refer to [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[Apache-2.0](LICENSE.txt)
