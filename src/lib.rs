//! Rust SDK for VMware Cloud Director.
//!
//! The crate provides a typed, synchronous client for VMware Cloud Director
//! endpoints and thin resource types for common vCloud entities. It focuses on
//! predictable request construction, explicit authentication, structured error
//! handling, XML/JSON payload support, and task monitoring.

pub mod api_version;
pub mod client;
pub mod error;
pub mod link;
pub mod query;
pub mod resource;
pub mod task;

pub use api_version::ApiVersion;
pub use client::{Auth, Client, ClientBuilder, Method, RequestBuilder, ResponseBody};
pub use error::{Error, Result, VcdError};
pub use link::Link;
pub use query::{Query, QueryPage, SortDirection};
pub use resource::{EntityType, Resource};
pub use task::{Task, TaskStatus};

macro_rules! entity_module {
    ($module:ident, $name:ident, $entity:ident) => {
        pub mod $module {
            use crate::{Client, EntityType, Resource, Result};

            #[derive(Clone, Debug)]
            pub struct $name {
                resource: Resource,
            }

            impl $name {
                pub fn new(client: Client, href: impl Into<String>) -> Self {
                    Self {
                        resource: Resource::new(client, EntityType::$entity, href),
                    }
                }

                pub fn from_resource(resource: Resource) -> Self {
                    Self { resource }
                }

                pub fn resource(&self) -> &Resource {
                    &self.resource
                }

                pub fn into_resource(self) -> Resource {
                    self.resource
                }

                pub fn get_xml(&self) -> Result<String> {
                    self.resource.get_xml()
                }

                pub fn get_json(&self) -> Result<serde_json::Value> {
                    self.resource.get_json()
                }

                pub fn update_xml(&self, body: impl Into<String>) -> Result<String> {
                    self.resource.update_xml(body)
                }

                pub fn update_json(&self, body: &serde_json::Value) -> Result<serde_json::Value> {
                    self.resource.update_json(body)
                }

                pub fn delete(&self) -> Result<Option<crate::Task>> {
                    self.resource.delete()
                }

                pub fn action_xml(&self, action: &str, body: impl Into<String>) -> Result<String> {
                    self.resource.action_xml(action, body)
                }

                pub fn metadata(&self) -> Result<String> {
                    self.resource.metadata()
                }
            }
        }
    };
}

entity_module!(acl, Acl, Acl);
entity_module!(amqp, Amqp, Amqp);
entity_module!(api_extension, ApiExtension, ApiExtension);
entity_module!(certificate, Certificate, Certificate);
entity_module!(crl, Crl, Crl);
entity_module!(dhcp_binding, DhcpBinding, DhcpBinding);
entity_module!(dhcp_pool, DhcpPool, DhcpPool);
entity_module!(external_network, ExternalNetwork, ExternalNetwork);
entity_module!(firewall_rule, FirewallRule, FirewallRule);
entity_module!(gateway, Gateway, Gateway);
entity_module!(gateway_services, GatewayServices, GatewayServices);
entity_module!(ipsec_vpn, IpsecVpn, IpsecVpn);
entity_module!(metadata, Metadata, Metadata);
entity_module!(nat_rule, NatRule, NatRule);
entity_module!(nsxt_extension, NsxtExtension, NsxtExtension);
entity_module!(org, Org, Org);
entity_module!(platform, Platform, Platform);
entity_module!(pvdc, ProviderVdc, ProviderVdc);
entity_module!(role, Role, Role);
entity_module!(static_route, StaticRoute, StaticRoute);
entity_module!(system, System, System);
entity_module!(vapp, VApp, VApp);
entity_module!(vapp_dhcp, VAppDhcp, VAppDhcp);
entity_module!(vapp_firewall, VAppFirewall, VAppFirewall);
entity_module!(vapp_nat, VAppNat, VAppNat);
entity_module!(vapp_services, VAppServices, VAppServices);
entity_module!(vapp_static_route, VAppStaticRoute, VAppStaticRoute);
entity_module!(vdc, Vdc, Vdc);
entity_module!(vdc_network, VdcNetwork, VdcNetwork);
entity_module!(vm, Vm, Vm);
