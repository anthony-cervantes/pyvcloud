//! Rust implementation for the pyvcloud client. This crate mirrors the
//! functionality provided by the deprecated Python SDK.

pub mod utils;

/// Prepare a base URI by ensuring the appropriate prefix and path components.
///
/// If `is_cloudapi` is true the "cloudapi" path is appended, otherwise "api"
/// is appended. The returned URI is ensured to start with `https://` if the
/// input did not already specify a scheme and will always end with the
/// relevant path component.
pub fn prep_base_uri(uri: &str, is_cloudapi: bool) -> String {
    let mut result = uri.to_string();
    if !result.is_empty() {
        if !result.ends_with('/') {
            result.push('/');
        }
        if is_cloudapi {
            result.push_str("cloudapi");
        } else {
            result.push_str("api");
        }
        if !result.starts_with("https://") && !result.starts_with("http://") {
            result = format!("https://{}", result);
        }
    }
    result
}

/// Simple client structure storing API base URIs.
#[derive(Debug, Clone)]
pub struct Client {
    api_base_uri: String,
    cloudapi_base_uri: String,
}

impl Client {
    /// Create a new client from a raw base URI.
    pub fn new(uri: &str) -> Self {
        Self {
            api_base_uri: prep_base_uri(uri, false),
            cloudapi_base_uri: prep_base_uri(uri, true),
        }
    }

    /// Get the API base URI.
    pub fn api_base_uri(&self) -> &str {
        &self.api_base_uri
    }

    /// Get the CloudAPI base URI.
    pub fn cloudapi_base_uri(&self) -> &str {
        &self.cloudapi_base_uri
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prep_base_uri_http_scheme() {
        let uri = prep_base_uri("example.com", false);
        assert_eq!(uri, "https://example.com/api");
    }

    #[test]
    fn client_uses_prep_base_uri() {
        let client = Client::new("vcd.example.com");
        assert_eq!(client.api_base_uri(), "https://vcd.example.com/api");
        assert_eq!(
            client.cloudapi_base_uri(),
            "https://vcd.example.com/cloudapi"
        );
    }
}
