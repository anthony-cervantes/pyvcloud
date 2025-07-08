//! Utility functions mirroring deprecated Python helpers.

/// Extract the ID portion of a vCloud urn string.
///
/// Example:
/// `urn:vcloud:catalog:39867ab4-04e0-4b13-b468-08abcc1de810` becomes
/// `39867ab4-04e0-4b13-b468-08abcc1de810`.
///
/// If `urn` is `None` this returns `None`.
pub fn extract_id(urn: Option<&str>) -> Option<String> {
    urn.map(|u| u.rsplit(':').next().unwrap_or(u).to_string())
}

/// Convert an IPv4 CIDR string to a gateway IP and netmask pair.
///
/// Returns `None` if the CIDR is malformed or contains an invalid prefix.
pub fn cidr_to_netmask(cidr: &str) -> Option<(String, String)> {
    let mut parts = cidr.split('/');
    let ip = parts.next()?.to_string();
    let prefix: u8 = parts.next()?.parse().ok()?;
    if prefix > 32 {
        return None;
    }
    let mask: u32 = if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    };
    let netmask = format!(
        "{}.{}.{}.{}",
        (mask >> 24) & 0xff,
        (mask >> 16) & 0xff,
        (mask >> 8) & 0xff,
        mask & 0xff
    );
    Some((ip, netmask))
}

/// Determine the CIDR prefix length from a network and netmask.
///
/// Returns `None` if the netmask is not contiguous.
pub fn netmask_to_cidr_prefix_len(_network: &str, netmask: &str) -> Option<u8> {
    let addr: std::net::Ipv4Addr = netmask.parse().ok()?;
    let mut mask = u32::from(addr);
    let mut count = 0;
    while mask & 0x8000_0000 != 0 {
        count += 1;
        mask <<= 1;
    }
    if mask != 0 {
        return None;
    }
    Some(count)
}

/// Convert a resource URI to its base API URI.
///
/// `https://host/api/vdc/123` becomes `https://host/api`.
pub fn uri_to_api_uri(uri: &str) -> Option<String> {
    let parts: Vec<&str> = uri.split('/').collect();
    if parts.len() < 4 {
        return None;
    }
    Some(parts[..4].join("/"))
}

#[cfg(test)]
mod tests {
    use super::{cidr_to_netmask, extract_id, netmask_to_cidr_prefix_len, uri_to_api_uri};

    #[test]
    fn none_returns_none() {
        assert_eq!(extract_id(None), None);
    }

    #[test]
    fn splits_urn() {
        let id = extract_id(Some("urn:vcloud:catalog:123"));
        assert_eq!(id.as_deref(), Some("123"));
    }

    #[test]
    fn returns_original_when_no_colon() {
        let id = extract_id(Some("abcd"));
        assert_eq!(id.as_deref(), Some("abcd"));
    }

    #[test]
    fn cidr_converts_to_netmask() {
        let result = cidr_to_netmask("10.2.2.1/20").unwrap();
        assert_eq!(result.0, "10.2.2.1");
        assert_eq!(result.1, "255.255.240.0");
    }

    #[test]
    fn netmask_converts_to_prefix_len() {
        let prefix = netmask_to_cidr_prefix_len("10.2.2.1", "255.255.240.0").unwrap();
        assert_eq!(prefix, 20);
    }

    #[test]
    fn uri_to_api_base() {
        let uri = uri_to_api_uri("https://10.150.198.98/api/vdc/123").unwrap();
        assert_eq!(uri, "https://10.150.198.98/api");
    }
}
