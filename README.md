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

These checks are also executed in GitHub Actions on every pull request.


## Notes

Please note that this project is under development and the interfaces might change over time.

`pyvcloud` is used by [vcd-cli](https://vmware.github.io/vcd-cli), the Command Line Interface for VMware vCloud Director. It requires Python 3.6 or higher.

Previous versions and deprecated code can be found in this repository under [tag 18.2.2](https://github.com/vmware/pyvcloud/tree/18.2.2).

## Migration to Rust

This Python implementation of `pyvcloud` is deprecated. Development is moving toward a Rust-based library that follows SOLID and DRY principles.
A small Rust crate lives under `pyvcloud-rs` providing a `Client` struct and helper utilities. Examples include `extract_id` for parsing URNs, `to_human` for formatting durations and `adapter_type_to_name` for displaying VM adapter types. The crate also defines an `ApiVersion` enum listing supported vCloud Director API versions. Additional enums such as `MetadataDomain`, `MetadataVisibility`, `TaskStatus`, `VAppPowerStatus`, `FenceMode`, `LogicalNetworkLinkType`, `NetworkAdapterType`, `RelationType`, `ResourceType`, `EntityType`, `WellKnownEndpoint` and `QueryResultFormat` model common vCD concepts.
A struct `VcdApiVersion` provides comparison logic for pre-release API versions
matching the behavior of the old Python SDK. A helper
`vcd_api_current_versions` returns the list of supported `VcdApiVersion`
objects.
Networking helpers like `cidr_to_netmask`, `uri_to_api_uri`, `build_network_url_from_gateway_url` and `retrieve_compute_policy_id_from_href` have also been ported as part of the migration. A `BasicLoginCredentials` struct stores user, organization and password values. Constants such as `SIZE_1MB`, `SYSTEM_ORG_NAME` and `ALPHA_API_SUBSTRING` mirror common values from the Python client.
A utility `filter_attributes` returns common attribute names for selected `ResourceType` values.
A `compute_policy` module offers constants such as `VDC_COMPUTE_POLICY_MIN_API_VERSION` and a helper `generate_compute_policy_tags` for constructing compute policy XML snippets.
A `network_constants` module exposes REST endpoint templates such as `FIREWALL_URL_TEMPLATE` for constructing URLs programmatically.
A helper `parse_supported_versions` can read the XML from the `/versions` endpoint and `fetch_supported_versions` retrieves it over HTTP using `reqwest`.
A helper function `get_safe_members_in_tar_file` is available for safely
extracting archives. Utility `extract_metadata_value` reads an XML snippet to
return the contained metadata text. Utility `to_camel_case` assists with case-insensitive name
matching. The `stdout_xml` helper prints XML to the console with optional
highlighting.
See [RUST_MIGRATION_CHECKLIST.md](RUST_MIGRATION_CHECKLIST.md) for an overview of the migration plan and progress.

## Contributing

The `pyvcloud` project team welcomes contributions from the community. Before you start working with `pyvcloud`, please read our [Developer Certificate of Origin](https://cla.vmware.com/dco). All contributions to this repository must be signed as described on that page. Your signature certifies that you wrote the patch or have the right to pass it on as an open-source patch. For more detailed information, refer to [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[Apache-2.0](LICENSE.txt)
