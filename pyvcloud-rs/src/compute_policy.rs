//! Utilities related to compute policy handling.

/// Minimum API version that supports compute policies.
pub const VDC_COMPUTE_POLICY_MIN_API_VERSION: f32 = 32.0;
/// Maximum API version using the legacy `VdcComputePolicy` element.
pub const VDC_COMPUTE_POLICY_MAX_API_VERSION: f32 = 33.0;
/// Minimum API version that supports VM sizing policy.
pub const VM_SIZING_POLICY_MIN_API_VERSION: f32 = 33.0;

use crate::utils::retrieve_compute_policy_id_from_href;

/// Generate compute policy XML snippets based on the API version.
///
/// The returned tuple represents the legacy `VdcComputePolicy` element and
/// the modern `ComputePolicy` element respectively. If an element is not
/// applicable `None` is returned in its place.
pub fn generate_compute_policy_tags(
    api_version: f32,
    sizing_policy_href: Option<&str>,
    placement_policy_href: Option<&str>,
) -> (Option<String>, Option<String>) {
    if api_version < VDC_COMPUTE_POLICY_MIN_API_VERSION
        || (sizing_policy_href.is_none() && placement_policy_href.is_none())
    {
        return (None, None);
    }

    if api_version < VDC_COMPUTE_POLICY_MAX_API_VERSION {
        let href = placement_policy_href.or(sizing_policy_href).unwrap();
        let id = retrieve_compute_policy_id_from_href(href).unwrap_or_default();
        let xml = format!(
            "<VdcComputePolicy href=\"{}\" id=\"{}\" type=\"application/json\" />",
            href, id
        );
        (Some(xml), None)
    } else {
        let mut xml = String::from("<ComputePolicy>");
        if let Some(href) = placement_policy_href {
            if let Some(id) = retrieve_compute_policy_id_from_href(href) {
                xml.push_str(&format!(
                    "<VmPlacementPolicy href=\"{}\" id=\"{}\" />",
                    href, id
                ));
            }
        }
        if let Some(href) = sizing_policy_href {
            if let Some(id) = retrieve_compute_policy_id_from_href(href) {
                xml.push_str(&format!(
                    "<VmSizingPolicy href=\"{}\" id=\"{}\" />",
                    href, id
                ));
            }
        }
        xml.push_str("</ComputePolicy>");
        if xml == "<ComputePolicy></ComputePolicy>" {
            (None, None)
        } else {
            (None, Some(xml))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_vdc_compute_policy() {
        let (vdc, compute) =
            generate_compute_policy_tags(32.0, Some("https://x/policies/s1"), None);
        assert!(compute.is_none());
        assert_eq!(
            vdc.unwrap(),
            "<VdcComputePolicy href=\"https://x/policies/s1\" id=\"s1\" type=\"application/json\" />"
        );
    }

    #[test]
    fn modern_compute_policy() {
        let (vdc, compute) = generate_compute_policy_tags(
            34.0,
            Some("https://x/policies/s1"),
            Some("https://x/policies/p1"),
        );
        assert!(vdc.is_none());
        assert_eq!(
            compute.unwrap(),
            "<ComputePolicy><VmPlacementPolicy href=\"https://x/policies/p1\" id=\"p1\" /><VmSizingPolicy href=\"https://x/policies/s1\" id=\"s1\" /></ComputePolicy>"
        );
    }
}
