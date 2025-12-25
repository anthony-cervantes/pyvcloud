# pyvcloud (Rust)

`pyvcloud` is now a Rust-native SDK and command-line utility for interacting with VMware vCloud Director. The crate focuses on predictable error handling, a small surface area, and adherence to modern Rust coding standards.

## Features

- Blocking client built on `reqwest` for environments that prefer synchronous flows.
- Helpers for authenticating with credentials or pre-issued tokens.
- Convenience functions for health checks and listing organizations.
- A CLI binary (`pyvcloud`) offering login, health-check, and organization listing helpers.

## Getting started

### Prerequisites

- Rust toolchain 1.75+ (install via [rustup](https://rustup.rs)).

### Build from source

```bash
# clone the repository
git clone https://github.com/vmware/pyvcloud.git
cd pyvcloud

# compile the crate and run the test suite
cargo build
cargo test
```

### Using the CLI

```bash
# show help
cargo run -- --help

# verify connectivity to the API root
cargo run -- --base-url https://vcloud.example.com health-check

# authenticate with credentials and print the returned token
cargo run -- --base-url https://vcloud.example.com login admin "SuperSecretPassword"

# list organizations using an existing token
cargo run -- --base-url https://vcloud.example.com --token abc123 list-orgs
```

Environment variables are available for convenience:

- `PVCLOUD_BASE_URL`
- `PVCLOUD_ORG`
- `PVCLOUD_TOKEN`

### Library usage

Add `pyvcloud` as a dependency in your `Cargo.toml` and construct the client directly:

```rust
use pyvcloud::VCloudClient;

fn main() -> Result<(), pyvcloud::ClientError> {
    let mut client = VCloudClient::new("https://vcloud.example.com", "System")?;
    let token = client.login_with_credentials("admin", "password")?;

    println!("Authenticated token: {}", token);

    let orgs = client.list_organizations()?;
    for org in orgs {
        println!("{} ({})", org.name, org.id);
    }

    Ok(())
}
```

## Contributing

The `pyvcloud` project team welcomes contributions from the community. Before you start working with `pyvcloud`, please read our [Developer Certificate of Origin](https://cla.vmware.com/dco). All contributions to this repository must be signed as described on that page. Your signature certifies that you wrote the patch or have the right to pass it on as an open-source patch. For more detailed information, refer to [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[Apache-2.0](LICENSE.txt)
