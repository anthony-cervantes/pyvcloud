# pyvcloud

`pyvcloud` is now a Rust SDK for VMware Cloud Director. The deprecated Python
implementation has been removed from the active source tree and replaced with a
Cargo library that provides:

- a synchronous VMware Cloud Director HTTP client;
- versioned XML and JSON request helpers;
- typed error mapping for vCloud Director responses;
- task parsing and polling;
- query construction; and
- resource wrappers for the entities that were represented by the legacy SDK,
  including organizations, VDCs, vApps, VMs, gateways, roles, provider VDCs,
  metadata, certificates, AMQP settings, NSX-T extension resources, NAT rules,
  DHCP resources, firewall rules, IPsec VPNs, static routes, and vApp services.

Supported API versions are 29.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0, and
37.0.0-alpha. The default API version is 36.0.

## Building and testing

Install a recent Rust toolchain and run:

```shell
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

## Example

```rust
use pyvcloud::{Client, Query, SortDirection};

fn main() -> pyvcloud::Result<()> {
    let client = Client::builder("https://vcd.example/")?
        .bearer_token("token")
        .build()?;

    let orgs = Query::new("org")
        .sort("name", SortDirection::Asc)
        .execute_xml(&client)?;

    println!("{}", orgs.body);
    Ok(())
}
```

A runnable version is available at `examples/list_orgs.rs` and expects `VCD_URL`
and `VCD_TOKEN` environment variables.

## Migration notes for Python users

The previous Python package, `setup.py`, Python examples, and Python tests have
been removed. Consumers should depend on this crate from Cargo and construct a
`Client` with either a bearer token or a basic-auth session login. Entity-specific
modules expose concrete wrappers around vCloud resources while preserving access
to generic XML, JSON, action, metadata, update, and delete operations.

## Contributing

Before contributing, read `CONTRIBUTING.md` and run the Cargo checks above.
Keep changes focused, documented, formatted, lint-clean, and covered by tests.

## License

[Apache-2.0](LICENSE.txt)
