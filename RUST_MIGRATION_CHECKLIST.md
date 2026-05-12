# Rust Migration Checklist

The active source tree has been migrated from the deprecated Python SDK to a Rust
Cargo library.

- [x] **Assess Current Python Implementation**
  - Reviewed legacy modules, tests, documentation, packaging, and examples.
  - Preserved the core API surfaces as Rust client, resource, query, task, link,
    API-version, and error modules.
- [x] **Define the Rust Project Structure**
  - Added a Cargo library crate.
  - Added Rust modules for client construction, API versions, response errors,
    queries, resources, links, and tasks.
- [x] **Establish Build & Test Workflow**
  - Cargo now drives formatting, linting, and tests.
  - Required local checks are `cargo fmt -- --check`, `cargo clippy -- -D warnings`,
    and `cargo test`.
- [x] **Port Core Functionality**
  - Replaced Python source with Rust request helpers, typed resources, task
    handling, API-version validation, query building, and response errors.
- [x] **Wrap REST API Interactions**
  - Added a synchronous `reqwest` client with bearer-token, basic-auth session,
    versioned XML, JSON, update, action, metadata, delete, and generic request
    support.
- [x] **Maintain Documentation**
  - Updated README and docs to describe Rust build, test, usage, and migration
    behavior.
- [x] **Verify Feature Parity Path**
  - Replaced legacy Python entity modules with Rust resource wrappers for the
    same VMware Cloud Director entity families.
- [x] **Package and Publish Readiness**
  - Added crate metadata, license metadata, README metadata, and Cargo lockfile.
- [x] **Remove Deprecated Python Implementation**
  - Removed Python package, Python tests, Python examples, and Python packaging
    files from the active source tree.
