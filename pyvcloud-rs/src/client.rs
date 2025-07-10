use std::collections::HashMap;
use std::fmt;

use crate::types::{WELL_KNOWN_ENDPOINTS, WellKnownEndpoint};

/// Constant representing one megabyte.
pub const SIZE_1MB: usize = 1024 * 1024;

/// Name of the system organization.
pub const SYSTEM_ORG_NAME: &str = "system";

/// Substring used for alpha API versions.
pub const ALPHA_API_SUBSTRING: &str = "alpha";

/// Standard HTTP header names used by the client.
pub const HEADER_ACCEPT_NAME: &str = "Accept";
pub const HEADER_AUTHORIZATION_NAME: &str = "Authorization";
pub const HEADER_CONNECTION_NAME: &str = "Connection";
pub const HEADER_CONTENT_LENGTH_NAME: &str = "Content-Length";
pub const HEADER_CONTENT_RANGE_NAME: &str = "Content-Range";
pub const HEADER_CONTENT_TYPE_NAME: &str = "Content-Type";
pub const HEADER_REQUEST_ID_NAME: &str = "X-VMWARE-VCLOUD-REQUEST-ID";
pub const HEADER_X_VCLOUD_AUTH_NAME: &str = "x-vcloud-authorization";
pub const HEADER_X_VMWARE_CLOUD_ACCESS_TOKEN_NAME: &str = "x-vmware-vcloud-access-token";

/// Value used to close HTTP connections.
pub const HEADER_CONNECTION_VALUE_CLOSE: &str = "close";

/// Headers whose values should be redacted when logging.
pub const HEADERS_TO_REDACT: &[&str] = &[
    HEADER_AUTHORIZATION_NAME,
    HEADER_X_VCLOUD_AUTH_NAME,
    HEADER_X_VMWARE_CLOUD_ACCESS_TOKEN_NAME,
];

/// Maximum number of retries when uploading file fragments.
pub const UPLOAD_FRAGMENT_MAX_RETRIES: u8 = 5;

/// Return a copy of `headers` with sensitive values replaced by "[REDACTED]".
pub fn redact_headers(
    headers: &std::collections::HashMap<String, String>,
) -> std::collections::HashMap<String, String> {
    headers
        .iter()
        .map(|(k, v)| {
            if HEADERS_TO_REDACT.iter().any(|h| h.eq_ignore_ascii_case(k)) {
                (k.clone(), "[REDACTED]".to_string())
            } else {
                (k.clone(), v.clone())
            }
        })
        .collect()
}

/// Basic set of credentials containing organisation, user and password.
#[derive(Debug, Clone)]
pub struct BasicLoginCredentials {
    pub user: String,
    pub org: String,
    pub password: String,
}

impl BasicLoginCredentials {
    /// Create a new credentials struct.
    pub fn new<U: Into<String>, O: Into<String>, P: Into<String>>(
        user: U,
        org: O,
        password: P,
    ) -> Self {
        Self {
            user: user.into(),
            org: org.into(),
            password: password.into(),
        }
    }
}

impl fmt::Display for BasicLoginCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.user, self.org)
    }
}

/// Extract well known session endpoints from a `<Session>` XML document.
///
/// The returned map is keyed by `WellKnownEndpoint` values with the
/// corresponding link `href` attribute as the value.
pub fn get_session_endpoints(
    xml: &str,
) -> Result<HashMap<WellKnownEndpoint, String>, roxmltree::Error> {
    let doc = roxmltree::Document::parse(xml)?;
    let mut map = HashMap::new();
    for endpoint in WELL_KNOWN_ENDPOINTS {
        let rel = endpoint.relation().to_string();
        let media_type = endpoint.entity().as_str();
        if let Some(link) = doc.descendants().find(|n| {
            n.has_tag_name("Link")
                && n.attribute("rel") == Some(rel.as_str())
                && n.attribute("type") == Some(media_type)
        }) {
            if let Some(href) = link.attribute("href") {
                map.insert(*endpoint, href.to_string());
            }
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_obscures_password() {
        let creds = BasicLoginCredentials::new("user", "org", "secret");
        assert_eq!(creds.to_string(), "user@org");
    }

    #[test]
    fn redact_sensitive_headers() {
        let mut headers = std::collections::HashMap::new();
        headers.insert(HEADER_AUTHORIZATION_NAME.to_string(), "token".to_string());
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        let redacted = redact_headers(&headers);
        assert_eq!(
            redacted.get(HEADER_AUTHORIZATION_NAME).unwrap(),
            "[REDACTED]"
        );
        assert_eq!(redacted.get("Content-Type").unwrap(), "text/plain");
    }

    #[test]
    fn parse_session_endpoints() {
        let xml = r#"
        <Session xmlns="http://www.vmware.com/vcloud/v1.5">
            <Link rel="down" type="application/vnd.vmware.vcloud.org+xml" href="https://host/api/org/1" />
            <Link rel="down" type="application/vnd.vmware.vcloud.query.queryList+xml" href="https://host/api/query" />
        </Session>
        "#;
        let map = get_session_endpoints(xml).unwrap();
        assert_eq!(
            map.get(&WellKnownEndpoint::LoggedInOrg).unwrap(),
            "https://host/api/org/1"
        );
        assert_eq!(
            map.get(&WellKnownEndpoint::QueryList).unwrap(),
            "https://host/api/query"
        );
    }
}
