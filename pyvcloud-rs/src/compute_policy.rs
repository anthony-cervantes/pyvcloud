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

/// Update the compute policy portion of a VM XML document.
///
/// The provided XML must represent a `<Vm>` element. When compute policy
/// information is added or modified, the function returns `true` and the VM
/// element is updated in place. When no change is required `false` is returned.
pub fn update_vm_compute_policy_element(
    api_version: f32,
    vm: &mut xmltree::Element,
    sizing_policy_href: Option<&str>,
    sizing_policy_final: bool,
    placement_policy_href: Option<&str>,
    placement_policy_final: bool,
) -> bool {
    if api_version < VM_SIZING_POLICY_MIN_API_VERSION
        && sizing_policy_href.is_none()
        && placement_policy_href.is_none()
    {
        return false;
    }

    let mut updated = false;
    let date_index = vm
        .children
        .iter()
        .position(|c| matches!(c, xmltree::XMLNode::Element(e) if e.name == "DateCreated"));

    let mut cp_index = vm
        .children
        .iter()
        .position(|c| matches!(c, xmltree::XMLNode::Element(e) if e.name == "ComputePolicy"));

    if cp_index.is_none() {
        let cp_elem = xmltree::Element::new("ComputePolicy");
        let insert_at = date_index.unwrap_or(vm.children.len());
        vm.children
            .insert(insert_at, xmltree::XMLNode::Element(cp_elem));
        cp_index = Some(insert_at);
        updated = true;
    }

    let cp_elem = match vm.children.get_mut(cp_index.unwrap()) {
        Some(xmltree::XMLNode::Element(e)) => e,
        _ => unreachable!(),
    };

    if let Some(href) = placement_policy_href {
        let mut policy_elem = cp_elem
            .get_mut_child("VmPlacementPolicy")
            .cloned()
            .unwrap_or_else(|| {
                updated = true;
                xmltree::Element::new("VmPlacementPolicy")
            });
        if policy_elem.attributes.get("href") != Some(&href.to_string()) {
            policy_elem
                .attributes
                .insert("href".into(), href.to_string());
            updated = true;
        }
        let id = retrieve_compute_policy_id_from_href(href).unwrap_or_default();
        if policy_elem.attributes.get("id") != Some(&id) {
            policy_elem.attributes.insert("id".into(), id);
            updated = true;
        }
        policy_elem
            .attributes
            .insert("type".into(), "application/json".into());

        if cp_elem.get_mut_child("VmPlacementPolicy").is_none() {
            let pos = 0;
            cp_elem
                .children
                .insert(pos, xmltree::XMLNode::Element(policy_elem.clone()));
            cp_elem.children.insert(
                pos + 1,
                xmltree::XMLNode::Element(xmltree::Element {
                    prefix: None,
                    namespace: None,
                    namespaces: None,
                    name: "VmPlacementPolicyFinal".into(),
                    attributes: std::collections::HashMap::new(),
                    children: vec![xmltree::XMLNode::Text(if placement_policy_final {
                        "true".into()
                    } else {
                        "false".into()
                    })],
                }),
            );
        } else {
            *cp_elem.get_mut_child("VmPlacementPolicy").unwrap() = policy_elem;
        }
    }

    if let Some(href) = sizing_policy_href {
        let mut policy_elem = cp_elem
            .get_mut_child("VmSizingPolicy")
            .cloned()
            .unwrap_or_else(|| {
                updated = true;
                xmltree::Element::new("VmSizingPolicy")
            });
        if policy_elem.attributes.get("href") != Some(&href.to_string()) {
            policy_elem
                .attributes
                .insert("href".into(), href.to_string());
            updated = true;
        }
        let id = retrieve_compute_policy_id_from_href(href).unwrap_or_default();
        if policy_elem.attributes.get("id") != Some(&id) {
            policy_elem.attributes.insert("id".into(), id);
            updated = true;
        }
        policy_elem
            .attributes
            .insert("type".into(), "application/json".into());

        if cp_elem.get_mut_child("VmSizingPolicy").is_none() {
            cp_elem
                .children
                .push(xmltree::XMLNode::Element(policy_elem.clone()));
            cp_elem
                .children
                .push(xmltree::XMLNode::Element(xmltree::Element {
                    prefix: None,
                    namespace: None,
                    namespaces: None,
                    name: "VmSizingPolicyFinal".into(),
                    attributes: std::collections::HashMap::new(),
                    children: vec![xmltree::XMLNode::Text(if sizing_policy_final {
                        "true".into()
                    } else {
                        "false".into()
                    })],
                }));
        } else {
            *cp_elem.get_mut_child("VmSizingPolicy").unwrap() = policy_elem;
        }
    }

    updated
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

    #[test]
    fn update_vm_compute_policy() {
        let xml = "<Vm><DateCreated>now</DateCreated></Vm>";
        let mut elem = xmltree::Element::parse(xml.as_bytes()).unwrap();
        let changed = update_vm_compute_policy_element(
            34.0,
            &mut elem,
            Some("https://x/policies/s1"),
            true,
            Some("https://x/policies/p1"),
            false,
        );
        assert!(changed);
        let mut buf = Vec::new();
        elem.write(&mut buf).unwrap();
        let out = String::from_utf8(buf).unwrap();
        assert!(out.contains("VmPlacementPolicy"));
        assert!(out.contains("VmSizingPolicy"));

        // Reapply with same data should yield no changes
        let changed_again = update_vm_compute_policy_element(
            34.0,
            &mut elem,
            Some("https://x/policies/s1"),
            true,
            Some("https://x/policies/p1"),
            false,
        );
        assert!(!changed_again);
    }
}
