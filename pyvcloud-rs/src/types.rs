//! Enumerations and constants ported from the deprecated Python SDK.

/// Types of edge gateway backing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeGatewayType {
    NsxvBacked,
    NsxtBacked,
    NsxtImported,
}

impl std::fmt::Display for EdgeGatewayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EdgeGatewayType::NsxvBacked => "NSXV_BACKED",
            EdgeGatewayType::NsxtBacked => "NSXT_BACKED",
            EdgeGatewayType::NsxtImported => "NSXT_IMPORTED",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for EdgeGatewayType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NSXV_BACKED" => Ok(EdgeGatewayType::NsxvBacked),
            "NSXT_BACKED" => Ok(EdgeGatewayType::NsxtBacked),
            "NSXT_IMPORTED" => Ok(EdgeGatewayType::NsxtImported),
            _ => Err(()),
        }
    }
}

/// Modes for obtaining an IP address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpAddressMode {
    Dhcp,
    Pool,
    Manual,
    None,
}

impl std::fmt::Display for IpAddressMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            IpAddressMode::Dhcp => "DHCP",
            IpAddressMode::Pool => "POOL",
            IpAddressMode::Manual => "MANUAL",
            IpAddressMode::None => "NONE",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for IpAddressMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DHCP" => Ok(IpAddressMode::Dhcp),
            "POOL" => Ok(IpAddressMode::Pool),
            "MANUAL" => Ok(IpAddressMode::Manual),
            "NONE" => Ok(IpAddressMode::None),
            _ => Err(()),
        }
    }
}

/// NIC property names for VMs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmNicProperty {
    Index,
    Connected,
    Primary,
    Network,
    IpAddressMode,
    IpAddress,
    AdapterType,
    MacAddress,
}

impl std::fmt::Display for VmNicProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            VmNicProperty::Index => "index",
            VmNicProperty::Connected => "connected",
            VmNicProperty::Primary => "primary",
            VmNicProperty::Network => "network",
            VmNicProperty::IpAddressMode => "ip_address_mode",
            VmNicProperty::IpAddress => "ip_address",
            VmNicProperty::AdapterType => "adapter_type",
            VmNicProperty::MacAddress => "mac_address",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for VmNicProperty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "index" => Ok(VmNicProperty::Index),
            "connected" => Ok(VmNicProperty::Connected),
            "primary" => Ok(VmNicProperty::Primary),
            "network" => Ok(VmNicProperty::Network),
            "ip_address_mode" => Ok(VmNicProperty::IpAddressMode),
            "ip_address" => Ok(VmNicProperty::IpAddress),
            "adapter_type" => Ok(VmNicProperty::AdapterType),
            "mac_address" => Ok(VmNicProperty::MacAddress),
            _ => Err(()),
        }
    }
}

/// Return the vCloud status message for a given status code.
pub fn vcloud_status_message(status: i32) -> Option<&'static str> {
    match status {
        -1 => Some("Could not be created"),
        0 => Some("Unresolved"),
        1 => Some("Resolved"),
        2 => Some("Deployed"),
        3 => Some("Suspended"),
        4 => Some("Powered on"),
        5 => Some("Waiting for user input"),
        6 => Some("Unknown state"),
        7 => Some("Unrecognized state"),
        8 => Some("Powered off"),
        9 => Some("Inconsistent state"),
        10 => Some("Children do not all have the same status"),
        11 => Some("Upload initiated, OVF descriptor pending"),
        12 => Some("Upload initiated, copying contents"),
        13 => Some("Upload initiated , disk contents pending"),
        14 => Some("Upload has been quarantined"),
        15 => Some("Upload quarantine period has expired"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_edge_gateway_type() {
        let t: EdgeGatewayType = "NSXT_BACKED".parse().unwrap();
        assert_eq!(t, EdgeGatewayType::NsxtBacked);
        assert_eq!(t.to_string(), "NSXT_BACKED");
    }

    #[test]
    fn parse_ip_address_mode() {
        let mode: IpAddressMode = "MANUAL".parse().unwrap();
        assert_eq!(mode, IpAddressMode::Manual);
        assert_eq!(mode.to_string(), "MANUAL");
    }

    #[test]
    fn parse_vm_nic_property() {
        let p: VmNicProperty = "adapter_type".parse().unwrap();
        assert_eq!(p, VmNicProperty::AdapterType);
        assert_eq!(p.to_string(), "adapter_type");
    }

    #[test]
    fn status_message_known() {
        assert_eq!(vcloud_status_message(4), Some("Powered on"));
    }

    #[test]
    fn status_message_unknown() {
        assert_eq!(vcloud_status_message(42), None);
    }
}
