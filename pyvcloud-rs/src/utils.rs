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

/// Build network URI for the NSX proxy API from a gateway URL.
///
/// Returns `None` if the input doesn't resemble an edge gateway URL.
pub fn build_network_url_from_gateway_url(gateway_href: &str) -> Option<String> {
    const NETWORK_URL: &str = "/network/edges/";
    const GATEWAY_API_URL: &str = "/api/edgeGateway/";
    const GATEWAY_ADMIN_API_URL: &str = "/api/admin/edgeGateway/";

    if gateway_href.contains(GATEWAY_API_URL) || gateway_href.contains(GATEWAY_ADMIN_API_URL) {
        let mut url = gateway_href.replace(GATEWAY_API_URL, NETWORK_URL);
        url = url.replace(GATEWAY_ADMIN_API_URL, NETWORK_URL);
        Some(url)
    } else {
        None
    }
}

/// Extract compute policy ID from its href.
pub fn retrieve_compute_policy_id_from_href(href: &str) -> Option<String> {
    href.rsplit('/').next().map(|s| s.to_string())
}

/// Convert a duration in seconds to a human readable weeks, days and hours
/// string in the form `"<weeks>w, <days>d, <hours>h"`.
pub fn to_human(seconds: u64) -> String {
    let weeks = seconds / (7 * 24 * 60 * 60);
    let days = seconds / (24 * 60 * 60) - 7 * weeks;
    let hours = seconds / (60 * 60) - 7 * 24 * weeks - 24 * days;
    format!("{}w, {}d, {}h", weeks, days, hours)
}

/// Map an adapter type number to a display string. Unknown numbers return a
/// placeholder message.
pub fn adapter_type_to_name(adapter_type: &str) -> String {
    match adapter_type {
        "1" => "IDE".to_string(),
        "2" => "BusLogic Parallel (SCSI)".to_string(),
        "3" => "LSI Logic Parallel (SCSI)".to_string(),
        "4" => "LSI Logic SAS (SCSI)".to_string(),
        "5" => "Paravirtual (SCSI)".to_string(),
        "6" => "SATA".to_string(),
        _ => format!("Adapter Type {}undefined", adapter_type),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        adapter_type_to_name, build_network_url_from_gateway_url, cidr_to_netmask, extract_id,
        netmask_to_cidr_prefix_len, retrieve_compute_policy_id_from_href, to_human, uri_to_api_uri,
    };

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

    #[test]
    fn build_network_url_from_gateway() {
        let url =
            build_network_url_from_gateway_url("https://host/api/admin/edgeGateway/uuid").unwrap();
        assert_eq!(url, "https://host/network/edges/uuid");
    }

    #[test]
    fn compute_policy_id_from_href() {
        let id = retrieve_compute_policy_id_from_href("https://x/policies/abc").unwrap();
        assert_eq!(id, "abc");
    }

    #[test]
    fn seconds_to_human_readable() {
        assert_eq!(to_human(10_800), "0w, 0d, 3h");
        assert_eq!(to_human(900_000), "1w, 3d, 10h");
    }

    #[test]
    fn adapter_type_lookup() {
        assert_eq!(adapter_type_to_name("3"), "LSI Logic Parallel (SCSI)".to_string());
        assert_eq!(adapter_type_to_name("9"), "Adapter Type 9undefined".to_string());
    }
}
