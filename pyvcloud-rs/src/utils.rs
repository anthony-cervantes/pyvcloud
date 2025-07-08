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

#[cfg(test)]
mod tests {
    use super::extract_id;

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
}
