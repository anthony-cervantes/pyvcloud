//! Constants mirroring network URL templates from the Python SDK.

/// Base firewall configuration path.
pub const FIREWALL_URL_TEMPLATE: &str = "/firewall/config";
/// Firewall rules collection path.
pub const FIREWALL_RULES_URL_TEMPLATE: &str = "/firewall/config/rules";
/// Firewall rule item path template.
pub const FIREWALL_RULE_URL_TEMPLATE: &str = "/firewall/config/rules/{0}";

/// Base DHCP configuration path.
pub const DHCP_URL_TEMPLATE: &str = "/dhcp/config";
/// DHCP pools collection path.
pub const DHCP_POOLS_URL_TEMPLATE: &str = "/dhcp/config/ippools";
/// DHCP pool item path template.
pub const DHCP_POOL_URL_TEMPLATE: &str = "/dhcp/config/ippools/{0}";

/// DHCP bindings collection path.
pub const DHCP_BINDINGS_URL_TEMPLATE: &str = "/dhcp/config/staticBindings";
/// DHCP binding item path template.
pub const DHCP_BINDING_URL_TEMPLATE: &str = "/dhcp/config/staticBindings/{0}";

/// Base NAT configuration path.
pub const NAT_URL_TEMPLATE: &str = "/nat/config";
/// NAT rules collection path.
pub const NAT_RULES_URL_TEMPLATE: &str = "/nat/config/rules";
/// NAT rule item path template.
pub const NAT_RULE_URL_TEMPLATE: &str = "/nat/config/rules/{0}";

/// Static routing configuration path.
pub const STATIC_ROUTE_URL_TEMPLATE: &str = "/routing/config/static";
/// IPsec VPN configuration path.
pub const IPSEC_VPN_URL_TEMPLATE: &str = "/ipsec/config";
/// Endpoint to upload service certificates.
pub const SERVICE_CERTIFICATE_POST: &str = "/services/truststore/certificate/";
/// Endpoint to upload certificate revocation lists.
pub const CRL_CERTIFICATE_POST: &str = "/services/truststore/crl/";
/// Endpoint to retrieve certificates scoped by resource.
pub const GET_CERTIFICATES: &str = "/services/truststore/certificate/scope/";
/// Endpoint to retrieve CRLs scoped by resource.
pub const GET_CRL_CERTIFICATES: &str = "/services/truststore/crl/scope/";
