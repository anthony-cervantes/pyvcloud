//! Utility functions mirroring deprecated Python helpers.

use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};

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

/// Return the canonical name from `names` matching `name` case-insensitively.
///
/// If no match is found `name` is returned as-is.
pub fn to_camel_case(name: &str, names: &[&str]) -> String {
    for n in names {
        if name.eq_ignore_ascii_case(n) {
            return (*n).to_string();
        }
    }
    name.to_string()
}

/// Return a string representation of XML with optional ANSI color
/// highlighting for element tags.
pub fn format_xml(xml: &str, colorized: bool) -> String {
    if !colorized {
        return xml.to_string();
    }
    let re = regex::Regex::new(r"</?[^>]+>").unwrap();
    let mut out = String::new();
    let mut last = 0;
    for mat in re.find_iter(xml) {
        out.push_str(&xml[last..mat.start()]);
        out.push_str("\x1b[34m");
        out.push_str(mat.as_str());
        out.push_str("\x1b[0m");
        last = mat.end();
    }
    out.push_str(&xml[last..]);
    out
}

/// Print XML to stdout with optional highlighting of element tags.
pub fn stdout_xml(xml: &str, colorized: bool) {
    println!("{}", format_xml(xml, colorized));
}

/// Normalize a path by removing `.` and `..` components without touching the
/// filesystem.
fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut result = PathBuf::new();
    for comp in path.as_ref().components() {
        match comp {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            other => result.push(other.as_os_str()),
        }
    }
    result
}

/// Determine if the given path would escape the provided base directory.
pub fn bad_path<P: AsRef<Path>, B: AsRef<Path>>(path: P, base: B) -> bool {
    let full = normalize_path(base.as_ref().join(path.as_ref()));
    !full.starts_with(base.as_ref())
}

/// Determine if a symlink or hard link target escapes the provided base
/// directory. `link_parent` is the parent directory of the link within the
/// archive.
pub fn bad_link<P: AsRef<Path>, B: AsRef<Path>, L: AsRef<Path>>(
    link_name: P,
    base: B,
    link_parent: L,
) -> bool {
    let parent = normalize_path(base.as_ref().join(link_parent.as_ref()));
    bad_path(link_name, parent)
}

/// Retrieve the list of entry paths in a tar archive that are safe to extract.
///
/// Entries with paths outside of `.` or with links pointing outside are
/// filtered out. The returned vector contains the safe entry paths as strings.
pub fn get_safe_members_in_tar_file<R: Read>(
    archive: &mut tar::Archive<R>,
) -> io::Result<Vec<String>> {
    let base = std::env::current_dir()?;
    let mut result = Vec::new();
    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?;
        if bad_path(&path, &base) {
            eprintln!("{} is blocked: illegal path.", path.display());
            continue;
        }
        let header = entry.header();
        if header.entry_type().is_symlink() || header.entry_type().is_hard_link() {
            if let Some(target) = entry.link_name()? {
                if bad_link(&target, &base, path.parent().unwrap_or(Path::new(""))) {
                    eprintln!(
                        "{} is blocked: link to {}",
                        path.display(),
                        target.display()
                    );
                    continue;
                }
            }
        }
        result.push(path.to_string_lossy().into_owned());
    }
    Ok(result)
}

/// Return an admin version of the given vCD URL.
///
/// If the input already points to the admin or admin extension endpoint it is
/// returned unchanged. Otherwise `/api/` is replaced with `/api/admin/`.
pub fn get_admin_href(href: &str) -> String {
    if href.contains("/api/admin/extension/") {
        href.replace("/api/admin/extension", "/api/admin/")
    } else if href.contains("/api/admin/") {
        href.to_string()
    } else {
        href.replace("/api/", "/api/admin/")
    }
}

/// Return a non-admin version of the given vCD URL.
///
/// Admin and admin extension paths are converted back to their non-admin form.
pub fn get_non_admin_href(href: &str) -> String {
    if href.contains("/api/admin/extension/") {
        href.replace("/api/admin/extension", "/api/")
    } else if href.contains("/api/admin/") {
        href.replace("/api/admin/", "/api/")
    } else {
        href.to_string()
    }
}

/// Determine if the URL refers to an admin endpoint.
pub fn is_admin(href: &str) -> bool {
    href.contains("/api/admin/") && !href.contains("/api/admin/extension/")
}

/// Return the sys admin extension version of the given vCD URL.
///
/// If the input already references the admin extension endpoint it is returned
/// unchanged.
pub fn get_admin_extension_href(href: &str) -> String {
    if href.contains("/api/admin/extension/") {
        href.to_string()
    } else if href.contains("/api/admin/") {
        href.replace("/api/admin/", "/api/admin/extension/")
    } else {
        href.replace("/api/", "/api/admin/extension/")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        adapter_type_to_name, build_network_url_from_gateway_url, cidr_to_netmask, extract_id,
        format_xml, get_admin_extension_href, get_admin_href, get_non_admin_href,
        get_safe_members_in_tar_file, is_admin, netmask_to_cidr_prefix_len,
        retrieve_compute_policy_id_from_href, to_camel_case, to_human, uri_to_api_uri,
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
        assert_eq!(
            adapter_type_to_name("3"),
            "LSI Logic Parallel (SCSI)".to_string()
        );
        assert_eq!(
            adapter_type_to_name("9"),
            "Adapter Type 9undefined".to_string()
        );
    }

    #[test]
    fn admin_href_conversion() {
        assert_eq!(
            get_admin_href("https://host/api/vdc/1"),
            "https://host/api/admin/vdc/1"
        );
        assert_eq!(
            get_admin_href("https://host/api/admin/vdc/1"),
            "https://host/api/admin/vdc/1"
        );
    }

    #[test]
    fn non_admin_href_conversion() {
        assert_eq!(
            get_non_admin_href("https://host/api/admin/vdc/1"),
            "https://host/api/vdc/1"
        );
        assert_eq!(
            get_non_admin_href("https://host/api/vdc/1"),
            "https://host/api/vdc/1"
        );
    }

    #[test]
    fn admin_detection() {
        assert!(is_admin("https://host/api/admin/vdc/1"));
        assert!(!is_admin("https://host/api/vdc/1"));
    }

    #[test]
    fn admin_extension_href_conversion() {
        assert_eq!(
            get_admin_extension_href("https://host/api/admin/vdc/1"),
            "https://host/api/admin/extension/vdc/1"
        );
        assert_eq!(
            get_admin_extension_href("https://host/api/admin/extension/vdc/1"),
            "https://host/api/admin/extension/vdc/1"
        );
    }

    #[test]
    fn safe_members_filters_illegal_paths() {
        use std::io::Cursor;
        use tar::{Archive, Builder, EntryType, Header};

        let mut data = Vec::new();
        {
            let mut builder = Builder::new(&mut data);

            let mut hdr = Header::new_gnu();
            hdr.set_path("safe.txt").unwrap();
            hdr.set_size(4);
            hdr.set_cksum();
            builder
                .append(&hdr, "safe".as_bytes())
                .expect("append safe");

            let mut hdr2 = Header::new_gnu();
            hdr2.set_entry_type(EntryType::Symlink);
            hdr2.set_path("link").unwrap();
            hdr2.set_size(0);
            hdr2.set_link_name_literal(b"../evil.txt").unwrap();
            hdr2.set_cksum();
            builder
                .append(&hdr2, std::io::empty())
                .expect("append link");

            builder.finish().unwrap();
        }

        let mut archive = Archive::new(Cursor::new(data));
        let members = get_safe_members_in_tar_file(&mut archive).unwrap();
        assert_eq!(members, vec![String::from("safe.txt")]);
    }

    #[test]
    fn camel_case_lookup() {
        let names = ["FooBar", "BazQuux"];
        assert_eq!(to_camel_case("foobar", &names), "FooBar".to_string());
        assert_eq!(to_camel_case("nomatch", &names), "nomatch".to_string());
    }

    #[test]
    fn format_xml_no_color() {
        let xml = "<foo></foo>";
        assert_eq!(format_xml(xml, false), xml);
    }

    #[test]
    fn format_xml_color_changes_output() {
        let xml = "<foo></foo>";
        assert_ne!(format_xml(xml, true), xml);
    }
}
