//! Handling of vCloud Director API version parsing and comparison.

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Representation of a vCloud Director API version.
///
/// Pre-release versions are considered equal if they share the same base
/// version and pre-release label regardless of the numeric suffix. This
/// mirrors the behaviour of the Python `VCDApiVersion` class.
#[derive(Debug, Clone)]
pub struct VcdApiVersion {
    major: u32,
    minor: u32,
    patch: u32,
    original: String,
    pre: Option<String>,
    label: Option<String>,
}

impl VcdApiVersion {
    fn base_cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }

    fn full_cmp(&self, other: &Self) -> Ordering {
        match self.base_cmp(other) {
            Ordering::Equal => match (&self.pre, &other.pre) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => a.cmp(b),
            },
            o => o,
        }
    }

    /// Return true if this is a pre-release version.
    pub fn is_prerelease(&self) -> bool {
        self.pre.is_some()
    }
}

impl FromStr for VcdApiVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let original = s.to_string();
        let (base, pre) = if let Some(idx) = s.find('-') {
            (&s[..idx], Some(&s[idx + 1..]))
        } else {
            (s, None)
        };
        let mut numbers = base.split('.');
        let major: u32 = numbers.next().ok_or(())?.parse().map_err(|_| ())?;
        let minor: u32 = numbers.next().unwrap_or("0").parse().map_err(|_| ())?;
        let patch: u32 = numbers.next().unwrap_or("0").parse().map_err(|_| ())?;
        let pre_owned = pre.map(|p| p.to_string());
        let label = pre.map(|p| p.split('-').next().unwrap_or(p).to_string());
        Ok(Self {
            major,
            minor,
            patch,
            original,
            pre: pre_owned,
            label,
        })
    }
}

impl fmt::Display for VcdApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.original)
    }
}

impl PartialEq for VcdApiVersion {
    fn eq(&self, other: &Self) -> bool {
        if self.is_prerelease()
            && other.is_prerelease()
            && self.base_cmp(other) == Ordering::Equal
            && self.label == other.label
        {
            true
        } else {
            self.major == other.major
                && self.minor == other.minor
                && self.patch == other.patch
                && self.pre == other.pre
        }
    }
}

impl Eq for VcdApiVersion {}

impl PartialOrd for VcdApiVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VcdApiVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_prerelease()
            && other.is_prerelease()
            && self.base_cmp(other) == Ordering::Equal
            && self.label == other.label
        {
            Ordering::Equal
        } else {
            self.full_cmp(other)
        }
    }
}

/// Return a list of `VcdApiVersion` objects representing the currently
/// supported API versions.
pub fn vcd_api_current_versions() -> Vec<VcdApiVersion> {
    vec![
        "29.0".parse().unwrap(),
        "30.0".parse().unwrap(),
        "31.0".parse().unwrap(),
        "32.0".parse().unwrap(),
        "33.0".parse().unwrap(),
        "34.0".parse().unwrap(),
        "35.0".parse().unwrap(),
        "36.0".parse().unwrap(),
        "37.0.0-alpha".parse().unwrap(),
    ]
}

/// Parse the XML returned by the `/versions` endpoint.
///
/// Versions marked as deprecated are skipped. When `include_alpha` is true,
/// `<AlphaVersion>` entries are also considered with any numeric suffix after
/// "alpha" removed.
pub fn parse_supported_versions(
    xml: &str,
    include_alpha: bool,
) -> Result<Vec<VcdApiVersion>, roxmltree::Error> {
    let doc = roxmltree::Document::parse(xml)?;
    let mut versions: Vec<VcdApiVersion> = Vec::new();

    for node in doc.descendants().filter(|n| n.has_tag_name("VersionInfo")) {
        if let Some(dep) = node.attribute("deprecated") {
            if dep != "false" {
                continue;
            }
        }
        if let Some(ver) = node
            .children()
            .find(|n| n.has_tag_name("Version"))
            .and_then(|n| n.text())
        {
            if let Ok(v) = ver.parse() {
                versions.push(v);
            }
        }
    }

    if include_alpha {
        for node in doc.descendants().filter(|n| n.has_tag_name("AlphaVersion")) {
            if let Some(dep) = node.attribute("deprecated") {
                if dep != "false" {
                    continue;
                }
            }
            if let Some(ver) = node
                .children()
                .find(|n| n.has_tag_name("Version"))
                .and_then(|n| n.text())
            {
                let mut text = ver.to_string();
                if let Some(idx) = text.find(crate::client::ALPHA_API_SUBSTRING) {
                    text.truncate(idx + crate::client::ALPHA_API_SUBSTRING.len());
                }
                if let Ok(v) = text.parse() {
                    versions.push(v);
                }
            }
        }
    }

    versions.sort();
    Ok(versions)
}

/// Retrieve the list of supported versions from a vCloud Director host.
pub fn fetch_supported_versions(
    base_uri: &str,
    include_alpha: bool,
) -> Result<Vec<VcdApiVersion>, Box<dyn Error>> {
    let url = format!("{}/versions", crate::prep_base_uri(base_uri, false));
    let resp = reqwest::blocking::get(url)?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()).into());
    }
    let text = resp.text()?;
    Ok(parse_supported_versions(&text, include_alpha)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prerelease_label_equality() {
        let a: VcdApiVersion = "37.0.0-alpha-1".parse().unwrap();
        let b: VcdApiVersion = "37.0.0-alpha-2".parse().unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn prerelease_vs_release() {
        let a: VcdApiVersion = "37.0.0-alpha-1".parse().unwrap();
        let b: VcdApiVersion = "37.0.0".parse().unwrap();
        assert!(a < b);
    }

    #[test]
    fn ordering_different_base() {
        let a: VcdApiVersion = "36.0".parse().unwrap();
        let b: VcdApiVersion = "37.0.0-alpha".parse().unwrap();
        assert!(a < b);
    }

    #[test]
    fn list_current_versions() {
        let versions = vcd_api_current_versions();
        assert_eq!(versions.len(), 9);
        assert_eq!(versions[0].to_string(), "29.0");
    }

    #[test]
    fn parse_versions_xml() {
        let xml = r#"
        <SupportedVersions>
            <VersionInfo deprecated="false"><Version>36.0</Version></VersionInfo>
            <AlphaVersion deprecated="false"><Version>37.0.0-alpha-1</Version></AlphaVersion>
        </SupportedVersions>
        "#;
        let versions = parse_supported_versions(xml, true).unwrap();
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].to_string(), "36.0");
        assert_eq!(versions[1].to_string(), "37.0.0-alpha");

        let versions_no_alpha = parse_supported_versions(xml, false).unwrap();
        assert_eq!(versions_no_alpha.len(), 1);
        assert_eq!(versions_no_alpha[0].to_string(), "36.0");
    }
}
