//! Rust-native client for interacting with VMware vCloud Director APIs.
//! The crate exposes a minimal, ergonomic surface for authenticating and
//! querying organizations while following idiomatic Rust patterns.

pub mod client;
pub mod error;
pub mod types;

pub use client::VCloudClient;
pub use error::ClientError;
pub use types::Organization;
