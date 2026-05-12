use std::fmt;

use serde_json::Value;

use crate::{Client, Method, Result, Task};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EntityType {
    Acl,
    Amqp,
    ApiExtension,
    Certificate,
    Crl,
    DhcpBinding,
    DhcpPool,
    ExternalNetwork,
    FirewallRule,
    Gateway,
    GatewayServices,
    IpsecVpn,
    Metadata,
    NatRule,
    NsxtExtension,
    Org,
    Platform,
    ProviderVdc,
    Role,
    StaticRoute,
    System,
    Task,
    VApp,
    VAppDhcp,
    VAppFirewall,
    VAppNat,
    VAppServices,
    VAppStaticRoute,
    Vdc,
    VdcNetwork,
    Vm,
}

impl EntityType {
    pub fn media_type(self) -> &'static str {
        match self {
            Self::Acl => "application/vnd.vmware.vcloud.accessControl+xml",
            Self::Amqp => "application/vnd.vmware.admin.amqpSettings+xml",
            Self::ApiExtension => "application/vnd.vmware.admin.apiExtension+xml",
            Self::Certificate => "application/vnd.vmware.admin.certificate+xml",
            Self::Crl => "application/vnd.vmware.admin.crl+xml",
            Self::DhcpBinding => "application/vnd.vmware.vcloud.dhcpBinding+xml",
            Self::DhcpPool => "application/vnd.vmware.vcloud.dhcpPool+xml",
            Self::ExternalNetwork => "application/vnd.vmware.admin.vmwexternalnet+xml",
            Self::FirewallRule => "application/vnd.vmware.vcloud.firewallRule+xml",
            Self::Gateway => "application/vnd.vmware.admin.edgeGateway+xml",
            Self::GatewayServices => "application/vnd.vmware.vcloud.gatewayFeatures+xml",
            Self::IpsecVpn => "application/vnd.vmware.vcloud.ipsecVpnTunnel+xml",
            Self::Metadata => "application/vnd.vmware.vcloud.metadata+xml",
            Self::NatRule => "application/vnd.vmware.vcloud.natRule+xml",
            Self::NsxtExtension => "application/json",
            Self::Org => "application/vnd.vmware.vcloud.org+xml",
            Self::Platform => "application/json",
            Self::ProviderVdc => "application/vnd.vmware.admin.vmwprovidervdc+xml",
            Self::Role => "application/vnd.vmware.admin.role+xml",
            Self::StaticRoute => "application/vnd.vmware.vcloud.staticRoute+xml",
            Self::System => "application/vnd.vmware.admin.vmwSystemOrganization+xml",
            Self::Task => "application/vnd.vmware.vcloud.task+xml",
            Self::VApp => "application/vnd.vmware.vcloud.vApp+xml",
            Self::VAppDhcp => "application/vnd.vmware.vcloud.dhcpService+xml",
            Self::VAppFirewall => "application/vnd.vmware.vcloud.firewallService+xml",
            Self::VAppNat => "application/vnd.vmware.vcloud.natService+xml",
            Self::VAppServices => "application/vnd.vmware.vcloud.networkFeatures+xml",
            Self::VAppStaticRoute => "application/vnd.vmware.vcloud.staticRoutingService+xml",
            Self::Vdc => "application/vnd.vmware.vcloud.vdc+xml",
            Self::VdcNetwork => "application/vnd.vmware.vcloud.orgVdcNetwork+xml",
            Self::Vm => "application/vnd.vmware.vcloud.vm+xml",
        }
    }
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Clone, Debug)]
pub struct Resource {
    client: Client,
    entity_type: EntityType,
    href: String,
}

impl Resource {
    pub fn new(client: Client, entity_type: EntityType, href: impl Into<String>) -> Self {
        Self {
            client,
            entity_type,
            href: href.into(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn href(&self) -> &str {
        &self.href
    }

    pub fn media_type(&self) -> &'static str {
        self.entity_type.media_type()
    }

    pub fn versioned_media_type(&self) -> String {
        format!(
            "{};version={}",
            self.media_type(),
            self.client.api_version()
        )
    }

    pub fn get_xml(&self) -> Result<String> {
        Ok(self
            .client
            .get(&self.href)
            .accept(self.versioned_media_type())
            .send()?
            .body)
    }

    pub fn get_json(&self) -> Result<Value> {
        self.client
            .get(&self.href)
            .accept("application/json")
            .send()?
            .json()
    }

    pub fn update_xml(&self, body: impl Into<String>) -> Result<String> {
        Ok(self
            .client
            .put(&self.href)
            .content_type(self.versioned_media_type())
            .accept(self.versioned_media_type())
            .body(body.into())
            .send()?
            .body)
    }

    pub fn update_json(&self, body: &Value) -> Result<Value> {
        self.client.put(&self.href).json(body)?.send()?.json()
    }

    pub fn create_child_xml(
        &self,
        relative_path: &str,
        entity_type: EntityType,
        body: impl Into<String>,
    ) -> Result<String> {
        let href = format!(
            "{}/{}",
            self.href.trim_end_matches('/'),
            relative_path.trim_start_matches('/')
        );
        Ok(self
            .client
            .post(href)
            .content_type(format!(
                "{};version={}",
                entity_type.media_type(),
                self.client.api_version()
            ))
            .body(body.into())
            .send()?
            .body)
    }

    pub fn action_xml(&self, action: &str, body: impl Into<String>) -> Result<String> {
        let href = format!("{}/action/{}", self.href.trim_end_matches('/'), action);
        Ok(self.client.post(href).xml(body).send()?.body)
    }

    pub fn delete(&self) -> Result<Option<Task>> {
        let response = self.client.request(Method::Delete, &self.href).send()?;
        if response.body.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(response.task()?))
        }
    }

    pub fn metadata(&self) -> Result<String> {
        let href = format!("{}/metadata", self.href.trim_end_matches('/'));
        Ok(self.client.get(href).send()?.body)
    }
}
