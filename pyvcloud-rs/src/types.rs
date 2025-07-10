//! Enumerations and constants ported from the deprecated Python SDK.

/// Types of edge gateway backing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeGatewayType {
    NsxvBacked,
    NsxtBacked,
    NsxtImported,
}

/// Resource type names used in query results and references.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    AclRule,
    AdminApiDefinition,
    AdminAllocatedExternalAddress,
    AdminCatalog,
    AdminCatalogItem,
    AdminDisk,
    AdminEvent,
    AdminFileDescriptor,
    AdminGroup,
    AdminMedia,
    AdminOrgNetwork,
    AdminOrgVdc,
    AdminOrgVdcStorageProfile,
    AdminRole,
    AdminService,
    AdminShadowVm,
    AdminTask,
    AdminUser,
    AdminVapp,
    AdminVappNetwork,
    AdminVappTemplate,
    AdminVm,
    AdminVmDiskRelation,
    AllocatedExternalAddress,
    ApiDefinition,
    ApiFilter,
    BlockingTask,
    Catalog,
    CatalogItem,
    Cell,
    Condition,
    Datastore,
    DatastoreProviderVdcRelation,
    Disk,
    DvSwitch,
    EdgeGateway,
    Event,
    ExternalLocalization,
    ExternalNetwork,
    FileDescriptor,
    FromCloudTunnel,
    Group,
    Host,
    Media,
    NetworkPool,
    NsxtManager,
    Organization,
    OrgNetwork,
    OrgVdc,
    OrgVdcNetwork,
    OrgVdcResourcePoolRelation,
    OrgVdcStorageProfile,
    PortGroup,
    ProviderVdc,
    ProviderVdcResourcePoolRelation,
    ProviderVdcStorageProfile,
    ResourceClass,
    ResourceClassAction,
    ResourcePool,
    ResourcePoolVmList,
    Right,
    Role,
    Service,
    ServiceLink,
    ServiceResource,
    StrandedItem,
    StrandedUser,
    Task,
    ToCloudTunnel,
    User,
    Vapp,
    VappNetwork,
    VappOrgVdcNetworkRelation,
    VappTemplate,
    VirtualCenter,
    Vm,
    VmDiskRelation,
    VmGroups,
    VmGroupVms,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::AclRule => "aclRule",
            ResourceType::AdminApiDefinition => "adminApiDefinition",
            ResourceType::AdminAllocatedExternalAddress => "adminAllocatedExternalAddress",
            ResourceType::AdminCatalog => "adminCatalog",
            ResourceType::AdminCatalogItem => "adminCatalogItem",
            ResourceType::AdminDisk => "adminDisk",
            ResourceType::AdminEvent => "adminEvent",
            ResourceType::AdminFileDescriptor => "adminFileDescriptor",
            ResourceType::AdminGroup => "adminGroup",
            ResourceType::AdminMedia => "adminMedia",
            ResourceType::AdminOrgNetwork => "adminOrgNetwork",
            ResourceType::AdminOrgVdc => "adminOrgVdc",
            ResourceType::AdminOrgVdcStorageProfile => "adminOrgVdcStorageProfile",
            ResourceType::AdminRole => "adminRole",
            ResourceType::AdminService => "adminService",
            ResourceType::AdminShadowVm => "adminShadowVM",
            ResourceType::AdminTask => "adminTask",
            ResourceType::AdminUser => "adminUser",
            ResourceType::AdminVapp => "adminVApp",
            ResourceType::AdminVappNetwork => "adminVAppNetwork",
            ResourceType::AdminVappTemplate => "adminVAppTemplate",
            ResourceType::AdminVm => "adminVM",
            ResourceType::AdminVmDiskRelation => "adminVMDiskRelation",
            ResourceType::AllocatedExternalAddress => "allocatedExternalAddress",
            ResourceType::ApiDefinition => "apiDefinition",
            ResourceType::ApiFilter => "apiFilter",
            ResourceType::BlockingTask => "blockingTask",
            ResourceType::Catalog => "catalog",
            ResourceType::CatalogItem => "catalogItem",
            ResourceType::Cell => "cell",
            ResourceType::Condition => "condition",
            ResourceType::Datastore => "datastore",
            ResourceType::DatastoreProviderVdcRelation => "datastoreProviderVdcRelation",
            ResourceType::Disk => "disk",
            ResourceType::DvSwitch => "dvSwitch",
            ResourceType::EdgeGateway => "edgeGateway",
            ResourceType::Event => "event",
            ResourceType::ExternalLocalization => "externalLocalization",
            ResourceType::ExternalNetwork => "externalNetwork",
            ResourceType::FileDescriptor => "fileDescriptor",
            ResourceType::FromCloudTunnel => "fromCloudTunnel",
            ResourceType::Group => "group",
            ResourceType::Host => "host",
            ResourceType::Media => "media",
            ResourceType::NetworkPool => "networkPool",
            ResourceType::NsxtManager => "nsxTManager",
            ResourceType::Organization => "organization",
            ResourceType::OrgNetwork => "orgNetwork",
            ResourceType::OrgVdc => "orgVdc",
            ResourceType::OrgVdcNetwork => "orgVdcNetwork",
            ResourceType::OrgVdcResourcePoolRelation => "orgVdcResourcePoolRelation",
            ResourceType::OrgVdcStorageProfile => "orgVdcStorageProfile",
            ResourceType::PortGroup => "portgroup",
            ResourceType::ProviderVdc => "providerVdc",
            ResourceType::ProviderVdcResourcePoolRelation => "providerVdcResourcePoolRelation",
            ResourceType::ProviderVdcStorageProfile => "providerVdcStorageProfile",
            ResourceType::ResourceClass => "resourceClass",
            ResourceType::ResourceClassAction => "resourceClassAction",
            ResourceType::ResourcePool => "resourcePool",
            ResourceType::ResourcePoolVmList => "resourcePoolVmList",
            ResourceType::Right => "right",
            ResourceType::Role => "role",
            ResourceType::Service => "service",
            ResourceType::ServiceLink => "serviceLink",
            ResourceType::ServiceResource => "serviceResource",
            ResourceType::StrandedItem => "strandedItem",
            ResourceType::StrandedUser => "strandedUser",
            ResourceType::Task => "task",
            ResourceType::ToCloudTunnel => "toCloudTunnel",
            ResourceType::User => "user",
            ResourceType::Vapp => "vApp",
            ResourceType::VappNetwork => "vAppNetwork",
            ResourceType::VappOrgVdcNetworkRelation => "vAppOrgVdcNetworkRelation",
            ResourceType::VappTemplate => "vAppTemplate",
            ResourceType::VirtualCenter => "virtualCenter",
            ResourceType::Vm => "vm",
            ResourceType::VmDiskRelation => "vmDiskRelation",
            ResourceType::VmGroups => "vmGroups",
            ResourceType::VmGroupVms => "vmGroupVms",
        }
    }
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for ResourceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aclRule" => Ok(ResourceType::AclRule),
            "adminApiDefinition" => Ok(ResourceType::AdminApiDefinition),
            "adminAllocatedExternalAddress" => Ok(ResourceType::AdminAllocatedExternalAddress),
            "adminCatalog" => Ok(ResourceType::AdminCatalog),
            "adminCatalogItem" => Ok(ResourceType::AdminCatalogItem),
            "adminDisk" => Ok(ResourceType::AdminDisk),
            "adminEvent" => Ok(ResourceType::AdminEvent),
            "adminFileDescriptor" => Ok(ResourceType::AdminFileDescriptor),
            "adminGroup" => Ok(ResourceType::AdminGroup),
            "adminMedia" => Ok(ResourceType::AdminMedia),
            "adminOrgNetwork" => Ok(ResourceType::AdminOrgNetwork),
            "adminOrgVdc" => Ok(ResourceType::AdminOrgVdc),
            "adminOrgVdcStorageProfile" => Ok(ResourceType::AdminOrgVdcStorageProfile),
            "adminRole" => Ok(ResourceType::AdminRole),
            "adminService" => Ok(ResourceType::AdminService),
            "adminShadowVM" => Ok(ResourceType::AdminShadowVm),
            "adminTask" => Ok(ResourceType::AdminTask),
            "adminUser" => Ok(ResourceType::AdminUser),
            "adminVApp" => Ok(ResourceType::AdminVapp),
            "adminVAppNetwork" => Ok(ResourceType::AdminVappNetwork),
            "adminVAppTemplate" => Ok(ResourceType::AdminVappTemplate),
            "adminVM" => Ok(ResourceType::AdminVm),
            "adminVMDiskRelation" => Ok(ResourceType::AdminVmDiskRelation),
            "allocatedExternalAddress" => Ok(ResourceType::AllocatedExternalAddress),
            "apiDefinition" => Ok(ResourceType::ApiDefinition),
            "apiFilter" => Ok(ResourceType::ApiFilter),
            "blockingTask" => Ok(ResourceType::BlockingTask),
            "catalog" => Ok(ResourceType::Catalog),
            "catalogItem" => Ok(ResourceType::CatalogItem),
            "cell" => Ok(ResourceType::Cell),
            "condition" => Ok(ResourceType::Condition),
            "datastore" => Ok(ResourceType::Datastore),
            "datastoreProviderVdcRelation" => Ok(ResourceType::DatastoreProviderVdcRelation),
            "disk" => Ok(ResourceType::Disk),
            "dvSwitch" => Ok(ResourceType::DvSwitch),
            "edgeGateway" => Ok(ResourceType::EdgeGateway),
            "event" => Ok(ResourceType::Event),
            "externalLocalization" => Ok(ResourceType::ExternalLocalization),
            "externalNetwork" => Ok(ResourceType::ExternalNetwork),
            "fileDescriptor" => Ok(ResourceType::FileDescriptor),
            "fromCloudTunnel" => Ok(ResourceType::FromCloudTunnel),
            "group" => Ok(ResourceType::Group),
            "host" => Ok(ResourceType::Host),
            "media" => Ok(ResourceType::Media),
            "networkPool" => Ok(ResourceType::NetworkPool),
            "nsxTManager" => Ok(ResourceType::NsxtManager),
            "organization" => Ok(ResourceType::Organization),
            "orgNetwork" => Ok(ResourceType::OrgNetwork),
            "orgVdc" => Ok(ResourceType::OrgVdc),
            "orgVdcNetwork" => Ok(ResourceType::OrgVdcNetwork),
            "orgVdcResourcePoolRelation" => Ok(ResourceType::OrgVdcResourcePoolRelation),
            "orgVdcStorageProfile" => Ok(ResourceType::OrgVdcStorageProfile),
            "portgroup" => Ok(ResourceType::PortGroup),
            "providerVdc" => Ok(ResourceType::ProviderVdc),
            "providerVdcResourcePoolRelation" => Ok(ResourceType::ProviderVdcResourcePoolRelation),
            "providerVdcStorageProfile" => Ok(ResourceType::ProviderVdcStorageProfile),
            "resourceClass" => Ok(ResourceType::ResourceClass),
            "resourceClassAction" => Ok(ResourceType::ResourceClassAction),
            "resourcePool" => Ok(ResourceType::ResourcePool),
            "resourcePoolVmList" => Ok(ResourceType::ResourcePoolVmList),
            "right" => Ok(ResourceType::Right),
            "role" => Ok(ResourceType::Role),
            "service" => Ok(ResourceType::Service),
            "serviceLink" => Ok(ResourceType::ServiceLink),
            "serviceResource" => Ok(ResourceType::ServiceResource),
            "strandedItem" => Ok(ResourceType::StrandedItem),
            "strandedUser" => Ok(ResourceType::StrandedUser),
            "task" => Ok(ResourceType::Task),
            "toCloudTunnel" => Ok(ResourceType::ToCloudTunnel),
            "user" => Ok(ResourceType::User),
            "vApp" => Ok(ResourceType::Vapp),
            "vAppNetwork" => Ok(ResourceType::VappNetwork),
            "vAppOrgVdcNetworkRelation" => Ok(ResourceType::VappOrgVdcNetworkRelation),
            "vAppTemplate" => Ok(ResourceType::VappTemplate),
            "virtualCenter" => Ok(ResourceType::VirtualCenter),
            "vm" => Ok(ResourceType::Vm),
            "vmDiskRelation" => Ok(ResourceType::VmDiskRelation),
            "vmGroups" => Ok(ResourceType::VmGroups),
            "vmGroupVms" => Ok(ResourceType::VmGroupVms),
            _ => Err(()),
        }
    }
}

pub const RESOURCE_TYPES: &[ResourceType] = &[
    ResourceType::AclRule,
    ResourceType::AdminApiDefinition,
    ResourceType::AdminAllocatedExternalAddress,
    ResourceType::AdminCatalog,
    ResourceType::AdminCatalogItem,
    ResourceType::AdminDisk,
    ResourceType::AdminEvent,
    ResourceType::AdminFileDescriptor,
    ResourceType::AdminGroup,
    ResourceType::AdminMedia,
    ResourceType::AdminOrgNetwork,
    ResourceType::AdminOrgVdc,
    ResourceType::AdminOrgVdcStorageProfile,
    ResourceType::AdminRole,
    ResourceType::AdminService,
    ResourceType::AdminShadowVm,
    ResourceType::AdminTask,
    ResourceType::AdminUser,
    ResourceType::AdminVapp,
    ResourceType::AdminVappNetwork,
    ResourceType::AdminVappTemplate,
    ResourceType::AdminVm,
    ResourceType::AdminVmDiskRelation,
    ResourceType::AllocatedExternalAddress,
    ResourceType::ApiDefinition,
    ResourceType::ApiFilter,
    ResourceType::BlockingTask,
    ResourceType::Catalog,
    ResourceType::CatalogItem,
    ResourceType::Cell,
    ResourceType::Condition,
    ResourceType::Datastore,
    ResourceType::DatastoreProviderVdcRelation,
    ResourceType::Disk,
    ResourceType::DvSwitch,
    ResourceType::EdgeGateway,
    ResourceType::Event,
    ResourceType::ExternalLocalization,
    ResourceType::ExternalNetwork,
    ResourceType::FileDescriptor,
    ResourceType::FromCloudTunnel,
    ResourceType::Group,
    ResourceType::Host,
    ResourceType::Media,
    ResourceType::NetworkPool,
    ResourceType::NsxtManager,
    ResourceType::Organization,
    ResourceType::OrgNetwork,
    ResourceType::OrgVdc,
    ResourceType::OrgVdcNetwork,
    ResourceType::OrgVdcResourcePoolRelation,
    ResourceType::OrgVdcStorageProfile,
    ResourceType::PortGroup,
    ResourceType::ProviderVdc,
    ResourceType::ProviderVdcResourcePoolRelation,
    ResourceType::ProviderVdcStorageProfile,
    ResourceType::ResourceClass,
    ResourceType::ResourceClassAction,
    ResourceType::ResourcePool,
    ResourceType::ResourcePoolVmList,
    ResourceType::Right,
    ResourceType::Role,
    ResourceType::Service,
    ResourceType::ServiceLink,
    ResourceType::ServiceResource,
    ResourceType::StrandedItem,
    ResourceType::StrandedUser,
    ResourceType::Task,
    ResourceType::ToCloudTunnel,
    ResourceType::User,
    ResourceType::Vapp,
    ResourceType::VappNetwork,
    ResourceType::VappOrgVdcNetworkRelation,
    ResourceType::VappTemplate,
    ResourceType::VirtualCenter,
    ResourceType::Vm,
    ResourceType::VmDiskRelation,
    ResourceType::VmGroups,
    ResourceType::VmGroupVms,
];
/// Entity type names used throughout the API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Admin,
    AdminCatalog,
    AdminOrg,
    AdminService,
    AllocatedNetworkAddress,
    AmqpSettings,
    ApiExtensibility,
    ApiFilter,
    ApplicationBinary,
    Catalog,
    CaptureVappParams,
    CheckPostGuestCustomizationSection,
    CloneVappParams,
    ComposeVappParams,
    ComplianceResult,
    ControlAccessParams,
    CurrentUsage,
    DatastoreReferences,
    DefaultContentType,
    Deploy,
    Disk,
    DiskAttachDetachParams,
    DiskCreateParms,
    EdgeGateway,
    EdgeGatewayFormFactor,
    EdgeGatewayServiceConfiguration,
    EdgeGatewaySysLogServerIp,
    Extension,
    ExtensionServices,
    ExternalNetwork,
    ExternalNetworkRefs,
    GuestCustomizationSection,
    HistoricUsage,
    Host,
    HostRefs,
    InstantiateVappTemplateParams,
    Json,
    LeaseSettings,
    Media,
    MediaInsertOrEjectParams,
    Metadata,
    MetadataValue,
    NetworkConfigSection,
    NetworkConnectionSection,
    NetworkManagers,
    NetworkPoolReferences,
    NsxtManager,
    OperatingSystemSection,
    Org,
    OrgNetwork,
    OrgList,
    OrgRights,
    OrgVdcNetwork,
    Owner,
    ProductSections,
    ProviderVdc,
    ProviderVdcParams,
    PublishCatalogParams,
    QueryList,
    RasdItem,
    RasdItemsList,
    RecomposeVappParams,
    Records,
    RegisterVcServerParams,
    RelocateParams,
    ResourcePoolList,
    ResPoolSetUpdateParams,
    Role,
    Right,
    Rights,
    SnapshotCreate,
    StartupSection,
    SystemSettings,
    Task,
    TasksList,
    TextXml,
    Undeploy,
    UpdateProviderVdcStorageProfiles,
    UpdateVdcStorageProfiles,
    UploadVappTemplateParams,
    User,
    Vapp,
    VappTemplate,
    Vdc,
    VdcAdmin,
    VdcComputePolicyReferences,
    VdcReferences,
    VdcStorageProfile,
    VdcStorageProfileAdmin,
    VdcsParams,
    VimServerRefs,
    VirtualCenter,
    Vm,
    VmBootOptions,
    VmCapabilitiesSection,
    Vms,
    VmScreenAcquireTicket,
    VmScreenAcquireMksticket,
    VmwProviderVdcResourcePool,
    VmwProviderVdcResourcePoolSet,
    VmwPvdcStorageProfile,
    VmwStorageProfiles,
    VappNetwork,
}

impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::Admin => "application/vnd.vmware.admin.vcloud+xml",
            EntityType::AdminCatalog => "application/vnd.vmware.admin.catalog+xml",
            EntityType::AdminOrg => "application/vnd.vmware.admin.organization+xml",
            EntityType::AdminService => "application/vnd.vmware.admin.service+xml",
            EntityType::AllocatedNetworkAddress => {
                "application/vnd.vmware.vcloud.allocatedNetworkAddress+xml"
            }
            EntityType::AmqpSettings => "application/vnd.vmware.admin.amqpSettings+xml",
            EntityType::ApiExtensibility => "application/vnd.vmware.vcloud.apiextensibility+xml",
            EntityType::ApiFilter => "application/vnd.vmware.admin.apiFilter+xml",
            EntityType::ApplicationBinary => "application/binary",
            EntityType::Catalog => "application/vnd.vmware.vcloud.catalog+xml",
            EntityType::CaptureVappParams => "application/vnd.vmware.vcloud.captureVAppParams+xml",
            EntityType::CheckPostGuestCustomizationSection => {
                "application/vnd.vmware.vcloud.vm.' 'checkPostGuestCustomizationSection+xml"
            }
            EntityType::CloneVappParams => "application/vnd.vmware.vcloud.cloneVAppParams+xml",
            EntityType::ComposeVappParams => "application/vnd.vmware.vcloud.composeVAppParams+xml",
            EntityType::ComplianceResult => "application/vnd.vmware.vm.complianceResult+xml",
            EntityType::ControlAccessParams => "application/vnd.vmware.vcloud.controlAccess+xml",
            EntityType::CurrentUsage => {
                "application/vnd.vmware.vcloud.metrics.currentUsageSpec+xml"
            }
            EntityType::DatastoreReferences => "application/vnd.vmware.admin.datastoreList+xml",
            EntityType::DefaultContentType => "application/*+xml",
            EntityType::Deploy => "application/vnd.vmware.vcloud.deployVAppParams+xml",
            EntityType::Disk => "application/vnd.vmware.vcloud.disk+xml",
            EntityType::DiskAttachDetachParams => {
                "application/vnd.vmware.vcloud.diskAttachOrDetachParams+xml"
            }
            EntityType::DiskCreateParms => "application/vnd.vmware.vcloud.diskCreateParams+xml",
            EntityType::EdgeGateway => "application/vnd.vmware.admin.edgeGateway+xml",
            EntityType::EdgeGatewayFormFactor => {
                "application/vnd.vmware.vcloud.edgeGatewayFormFactor+xml"
            }
            EntityType::EdgeGatewayServiceConfiguration => {
                "application/vnd.vmware.admin.edgeGatewayServiceConfiguration+xml"
            }
            EntityType::EdgeGatewaySysLogServerIp => {
                "application/vnd.vmware.vcloud.SyslogSettings+xml"
            }
            EntityType::Extension => "application/vnd.vmware.admin.vmwExtension+xml",
            EntityType::ExtensionServices => "application/vnd.vmware.admin.extensionServices+xml",
            EntityType::ExternalNetwork => "application/vnd.vmware.admin.vmwexternalnet+xml",
            EntityType::ExternalNetworkRefs => {
                "application/vnd.vmware.admin.vmwExternalNetworkReferences+xml"
            }
            EntityType::GuestCustomizationSection => {
                "application/vnd.vmware.vcloud.guestCustomizationSection+xml"
            }
            EntityType::HistoricUsage => {
                "application/vnd.vmware.vcloud.metrics.historicUsageSpec+xml"
            }
            EntityType::Host => "application/vnd.vmware.admin.host+xml",
            EntityType::HostRefs => "application/vnd.vmware.admin.vmwHostReferences+xml",
            EntityType::InstantiateVappTemplateParams => {
                "application/vnd.vmware.vcloud.instantiateVAppTemplateParams+xml"
            }
            EntityType::Json => "application/json",
            EntityType::LeaseSettings => "application/vnd.vmware.vcloud.leaseSettingsSection+xml",
            EntityType::Media => "application/vnd.vmware.vcloud.media+xml",
            EntityType::MediaInsertOrEjectParams => {
                "application/vnd.vmware.vcloud.mediaInsertOrEjectParams+xml"
            }
            EntityType::Metadata => "application/vnd.vmware.vcloud.metadata+xml",
            EntityType::MetadataValue => "application/vnd.vmware.vcloud.metadata.value+xml",
            EntityType::NetworkConfigSection => {
                "application/vnd.vmware.vcloud.networkConfigSection+xml"
            }
            EntityType::NetworkConnectionSection => {
                "application/vnd.vmware.vcloud.networkConnectionSection+xml"
            }
            EntityType::NetworkManagers => "application/vnd.vmware.admin.networkManagers+xml",
            EntityType::NetworkPoolReferences => {
                "application/vnd.vmware.admin.vmwNetworkPoolReferences+xml"
            }
            EntityType::NsxtManager => "application/vnd.vmware.admin.nsxTmanager+xml",
            EntityType::OperatingSystemSection => {
                "application/vnd.vmware.vcloud.operatingSystemSection+xml"
            }
            EntityType::Org => "application/vnd.vmware.vcloud.org+xml",
            EntityType::OrgNetwork => "application/vnd.vmware.vcloud.orgNetwork+xml",
            EntityType::OrgList => "application/vnd.vmware.vcloud.orgList+xml",
            EntityType::OrgRights => "application/vnd.vmware.admin.org.rights+xml",
            EntityType::OrgVdcNetwork => "application/vnd.vmware.vcloud.orgVdcNetwork+xml",
            EntityType::Owner => "application/vnd.vmware.vcloud.owner+xml",
            EntityType::ProductSections => "application/vnd.vmware.vcloud.productSections+xml",
            EntityType::ProviderVdc => "application/vnd.vmware.admin.providervdc+xml",
            EntityType::ProviderVdcParams => {
                "application/vnd.vmware.admin.createProviderVdcParams+xml"
            }
            EntityType::PublishCatalogParams => {
                "application/vnd.vmware.admin.publishCatalogParams+xml"
            }
            EntityType::QueryList => "application/vnd.vmware.vcloud.query.queryList+xml",
            EntityType::RasdItem => "application/vnd.vmware.vcloud.rasdItem+xml",
            EntityType::RasdItemsList => "application/vnd.vmware.vcloud.rasdItemsList+xml",
            EntityType::RecomposeVappParams => {
                "application/vnd.vmware.vcloud.recomposeVAppParams+xml"
            }
            EntityType::Records => "application/vnd.vmware.vcloud.query.records+xml",
            EntityType::RegisterVcServerParams => {
                "application/vnd.vmware.admin.registerVimServerParams+xml"
            }
            EntityType::RelocateParams => "application/vnd.vmware.vcloud.relocateVmParams+xml",
            EntityType::ResourcePoolList => "application/vnd.vmware.admin.resourcePoolList+xml",
            EntityType::ResPoolSetUpdateParams => {
                "application/vnd.vmware.admin.resourcePoolSetUpdateParams+xml"
            }
            EntityType::Role => "application/vnd.vmware.admin.role+xml",
            EntityType::Right => "application/vnd.vmware.admin.right+xml",
            EntityType::Rights => "application/vnd.vmware.admin.rights+xml",
            EntityType::SnapshotCreate => "application/vnd.vmware.vcloud.createSnapshotParams+xml",
            EntityType::StartupSection => "application/vnd.vmware.vcloud.startupSection+xml",
            EntityType::SystemSettings => "application/vnd.vmware.admin.systemSettings+xml",
            EntityType::Task => "application/vnd.vmware.vcloud.task+xml",
            EntityType::TasksList => "application/vnd.vmware.vcloud.tasksList+xml",
            EntityType::TextXml => "text/xml",
            EntityType::Undeploy => "application/vnd.vmware.vcloud.undeployVAppParams+xml",
            EntityType::UpdateProviderVdcStorageProfiles => {
                "application/vnd.vmware.admin.updateProviderVdcStorageProfiles+xml"
            }
            EntityType::UpdateVdcStorageProfiles => {
                "application/vnd.vmware.admin.updateVdcStorageProfiles+xml"
            }
            EntityType::UploadVappTemplateParams => {
                "application/vnd.vmware.vcloud.uploadVAppTemplateParams+xml"
            }
            EntityType::User => "application/vnd.vmware.admin.user+xml",
            EntityType::Vapp => "application/vnd.vmware.vcloud.vApp+xml",
            EntityType::VappTemplate => "application/vnd.vmware.vcloud.vAppTemplate+xml",
            EntityType::Vdc => "application/vnd.vmware.vcloud.vdc+xml",
            EntityType::VdcAdmin => "application/vnd.vmware.admin.vdc+xml",
            EntityType::VdcComputePolicyReferences => {
                "application/vnd.vmware.vcloud.vdcComputePolicyReferences+xml"
            }
            EntityType::VdcReferences => "application/vnd.vmware.admin.vdcReferences+xml",
            EntityType::VdcStorageProfile => "application/vnd.vmware.vcloud.vdcStorageProfile+xml",
            EntityType::VdcStorageProfileAdmin => {
                "application/vnd.vmware.admin.vdcStorageProfile+xml"
            }
            EntityType::VdcsParams => "application/vnd.vmware.admin.createVdcParams+xml",
            EntityType::VimServerRefs => "application/vnd.vmware.admin.vmwVimServerReferences+xml",
            EntityType::VirtualCenter => "application/vnd.vmware.admin.vmwvirtualcenter+xml",
            EntityType::Vm => "application/vnd.vmware.vcloud.vm+xml",
            EntityType::VmBootOptions => "application/vnd.vmware.vcloud.bootOptionsSection+xml",
            EntityType::VmCapabilitiesSection => {
                "application/vnd.vmware.vcloud.vmCapabilitiesSection+xml"
            }
            EntityType::Vms => "application/vnd.vmware.vcloud.vms+xml",
            EntityType::VmScreenAcquireTicket => "application/vnd.vmware.vcloud.screenTicket+xml",
            EntityType::VmScreenAcquireMksticket => "application/vnd.vmware.vcloud.mksTicket+xml",
            EntityType::VmwProviderVdcResourcePool => {
                "application/vnd.vmware.admin.vmwProviderVdcResourcePool+xml"
            }
            EntityType::VmwProviderVdcResourcePoolSet => {
                "application/vnd.vmware.admin.vmwProviderVdcResourcePoolSet+xml"
            }
            EntityType::VmwPvdcStorageProfile => {
                "application/vnd.vmware.admin.vmwPvdcStorageProfile+xml"
            }
            EntityType::VmwStorageProfiles => "application/vnd.vmware.admin.vmwStorageProfiles+xml",
            EntityType::VappNetwork => "application/vnd.vmware.vcloud.vAppNetwork+xml",
        }
    }
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for EntityType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "application/vnd.vmware.admin.vcloud+xml" => Ok(EntityType::Admin),
            "admin" => Ok(EntityType::Admin),
            "application/vnd.vmware.admin.catalog+xml" => Ok(EntityType::AdminCatalog),
            "admin_catalog" => Ok(EntityType::AdminCatalog),
            "application/vnd.vmware.admin.organization+xml" => Ok(EntityType::AdminOrg),
            "admin_org" => Ok(EntityType::AdminOrg),
            "application/vnd.vmware.admin.service+xml" => Ok(EntityType::AdminService),
            "admin_service" => Ok(EntityType::AdminService),
            "application/vnd.vmware.vcloud.allocatedNetworkAddress+xml" => {
                Ok(EntityType::AllocatedNetworkAddress)
            }
            "allocated_network_address" => Ok(EntityType::AllocatedNetworkAddress),
            "application/vnd.vmware.admin.amqpSettings+xml" => Ok(EntityType::AmqpSettings),
            "amqp_settings" => Ok(EntityType::AmqpSettings),
            "application/vnd.vmware.vcloud.apiextensibility+xml" => {
                Ok(EntityType::ApiExtensibility)
            }
            "api_extensibility" => Ok(EntityType::ApiExtensibility),
            "application/vnd.vmware.admin.apiFilter+xml" => Ok(EntityType::ApiFilter),
            "api_filter" => Ok(EntityType::ApiFilter),
            "application/binary" => Ok(EntityType::ApplicationBinary),
            "application_binary" => Ok(EntityType::ApplicationBinary),
            "application/vnd.vmware.vcloud.catalog+xml" => Ok(EntityType::Catalog),
            "catalog" => Ok(EntityType::Catalog),
            "application/vnd.vmware.vcloud.captureVAppParams+xml" => {
                Ok(EntityType::CaptureVappParams)
            }
            "capture_vapp_params" => Ok(EntityType::CaptureVappParams),
            "application/vnd.vmware.vcloud.vm.' 'checkPostGuestCustomizationSection+xml" => {
                Ok(EntityType::CheckPostGuestCustomizationSection)
            }
            "check_post_guest_customization_section" => {
                Ok(EntityType::CheckPostGuestCustomizationSection)
            }
            "application/vnd.vmware.vcloud.cloneVAppParams+xml" => Ok(EntityType::CloneVappParams),
            "clone_vapp_params" => Ok(EntityType::CloneVappParams),
            "application/vnd.vmware.vcloud.composeVAppParams+xml" => {
                Ok(EntityType::ComposeVappParams)
            }
            "compose_vapp_params" => Ok(EntityType::ComposeVappParams),
            "application/vnd.vmware.vm.complianceResult+xml" => Ok(EntityType::ComplianceResult),
            "compliance_result" => Ok(EntityType::ComplianceResult),
            "application/vnd.vmware.vcloud.controlAccess+xml" => {
                Ok(EntityType::ControlAccessParams)
            }
            "control_access_params" => Ok(EntityType::ControlAccessParams),
            "application/vnd.vmware.vcloud.metrics.currentUsageSpec+xml" => {
                Ok(EntityType::CurrentUsage)
            }
            "current_usage" => Ok(EntityType::CurrentUsage),
            "application/vnd.vmware.admin.datastoreList+xml" => Ok(EntityType::DatastoreReferences),
            "datastore_references" => Ok(EntityType::DatastoreReferences),
            "application/*+xml" => Ok(EntityType::DefaultContentType),
            "default_content_type" => Ok(EntityType::DefaultContentType),
            "application/vnd.vmware.vcloud.deployVAppParams+xml" => Ok(EntityType::Deploy),
            "deploy" => Ok(EntityType::Deploy),
            "application/vnd.vmware.vcloud.disk+xml" => Ok(EntityType::Disk),
            "disk" => Ok(EntityType::Disk),
            "application/vnd.vmware.vcloud.diskAttachOrDetachParams+xml" => {
                Ok(EntityType::DiskAttachDetachParams)
            }
            "disk_attach_detach_params" => Ok(EntityType::DiskAttachDetachParams),
            "application/vnd.vmware.vcloud.diskCreateParams+xml" => Ok(EntityType::DiskCreateParms),
            "disk_create_parms" => Ok(EntityType::DiskCreateParms),
            "application/vnd.vmware.admin.edgeGateway+xml" => Ok(EntityType::EdgeGateway),
            "edge_gateway" => Ok(EntityType::EdgeGateway),
            "application/vnd.vmware.vcloud.edgeGatewayFormFactor+xml" => {
                Ok(EntityType::EdgeGatewayFormFactor)
            }
            "edge_gateway_form_factor" => Ok(EntityType::EdgeGatewayFormFactor),
            "application/vnd.vmware.admin.edgeGatewayServiceConfiguration+xml" => {
                Ok(EntityType::EdgeGatewayServiceConfiguration)
            }
            "edge_gateway_service_configuration" => Ok(EntityType::EdgeGatewayServiceConfiguration),
            "application/vnd.vmware.vcloud.SyslogSettings+xml" => {
                Ok(EntityType::EdgeGatewaySysLogServerIp)
            }
            "edge_gateway_sys_log_server_ip" => Ok(EntityType::EdgeGatewaySysLogServerIp),
            "application/vnd.vmware.admin.vmwExtension+xml" => Ok(EntityType::Extension),
            "extension" => Ok(EntityType::Extension),
            "application/vnd.vmware.admin.extensionServices+xml" => {
                Ok(EntityType::ExtensionServices)
            }
            "extension_services" => Ok(EntityType::ExtensionServices),
            "application/vnd.vmware.admin.vmwexternalnet+xml" => Ok(EntityType::ExternalNetwork),
            "external_network" => Ok(EntityType::ExternalNetwork),
            "application/vnd.vmware.admin.vmwExternalNetworkReferences+xml" => {
                Ok(EntityType::ExternalNetworkRefs)
            }
            "external_network_refs" => Ok(EntityType::ExternalNetworkRefs),
            "application/vnd.vmware.vcloud.guestCustomizationSection+xml" => {
                Ok(EntityType::GuestCustomizationSection)
            }
            "guest_customization_section" => Ok(EntityType::GuestCustomizationSection),
            "application/vnd.vmware.vcloud.metrics.historicUsageSpec+xml" => {
                Ok(EntityType::HistoricUsage)
            }
            "historic_usage" => Ok(EntityType::HistoricUsage),
            "application/vnd.vmware.admin.host+xml" => Ok(EntityType::Host),
            "host" => Ok(EntityType::Host),
            "application/vnd.vmware.admin.vmwHostReferences+xml" => Ok(EntityType::HostRefs),
            "host_refs" => Ok(EntityType::HostRefs),
            "application/vnd.vmware.vcloud.instantiateVAppTemplateParams+xml" => {
                Ok(EntityType::InstantiateVappTemplateParams)
            }
            "instantiate_vapp_template_params" => Ok(EntityType::InstantiateVappTemplateParams),
            "application/json" => Ok(EntityType::Json),
            "json" => Ok(EntityType::Json),
            "application/vnd.vmware.vcloud.leaseSettingsSection+xml" => {
                Ok(EntityType::LeaseSettings)
            }
            "lease_settings" => Ok(EntityType::LeaseSettings),
            "application/vnd.vmware.vcloud.media+xml" => Ok(EntityType::Media),
            "media" => Ok(EntityType::Media),
            "application/vnd.vmware.vcloud.mediaInsertOrEjectParams+xml" => {
                Ok(EntityType::MediaInsertOrEjectParams)
            }
            "media_insert_or_eject_params" => Ok(EntityType::MediaInsertOrEjectParams),
            "application/vnd.vmware.vcloud.metadata+xml" => Ok(EntityType::Metadata),
            "metadata" => Ok(EntityType::Metadata),
            "application/vnd.vmware.vcloud.metadata.value+xml" => Ok(EntityType::MetadataValue),
            "metadata_value" => Ok(EntityType::MetadataValue),
            "application/vnd.vmware.vcloud.networkConfigSection+xml" => {
                Ok(EntityType::NetworkConfigSection)
            }
            "network_config_section" => Ok(EntityType::NetworkConfigSection),
            "application/vnd.vmware.vcloud.networkConnectionSection+xml" => {
                Ok(EntityType::NetworkConnectionSection)
            }
            "network_connection_section" => Ok(EntityType::NetworkConnectionSection),
            "application/vnd.vmware.admin.networkManagers+xml" => Ok(EntityType::NetworkManagers),
            "network_managers" => Ok(EntityType::NetworkManagers),
            "application/vnd.vmware.admin.vmwNetworkPoolReferences+xml" => {
                Ok(EntityType::NetworkPoolReferences)
            }
            "network_pool_references" => Ok(EntityType::NetworkPoolReferences),
            "application/vnd.vmware.admin.nsxTmanager+xml" => Ok(EntityType::NsxtManager),
            "nsxt_manager" => Ok(EntityType::NsxtManager),
            "application/vnd.vmware.vcloud.operatingSystemSection+xml" => {
                Ok(EntityType::OperatingSystemSection)
            }
            "operating_system_section" => Ok(EntityType::OperatingSystemSection),
            "application/vnd.vmware.vcloud.org+xml" => Ok(EntityType::Org),
            "org" => Ok(EntityType::Org),
            "application/vnd.vmware.vcloud.orgNetwork+xml" => Ok(EntityType::OrgNetwork),
            "org_network" => Ok(EntityType::OrgNetwork),
            "application/vnd.vmware.vcloud.orgList+xml" => Ok(EntityType::OrgList),
            "org_list" => Ok(EntityType::OrgList),
            "application/vnd.vmware.admin.org.rights+xml" => Ok(EntityType::OrgRights),
            "org_rights" => Ok(EntityType::OrgRights),
            "application/vnd.vmware.vcloud.orgVdcNetwork+xml" => Ok(EntityType::OrgVdcNetwork),
            "org_vdc_network" => Ok(EntityType::OrgVdcNetwork),
            "application/vnd.vmware.vcloud.owner+xml" => Ok(EntityType::Owner),
            "owner" => Ok(EntityType::Owner),
            "application/vnd.vmware.vcloud.productSections+xml" => Ok(EntityType::ProductSections),
            "product_sections" => Ok(EntityType::ProductSections),
            "application/vnd.vmware.admin.providervdc+xml" => Ok(EntityType::ProviderVdc),
            "provider_vdc" => Ok(EntityType::ProviderVdc),
            "application/vnd.vmware.admin.createProviderVdcParams+xml" => {
                Ok(EntityType::ProviderVdcParams)
            }
            "provider_vdc_params" => Ok(EntityType::ProviderVdcParams),
            "application/vnd.vmware.admin.publishCatalogParams+xml" => {
                Ok(EntityType::PublishCatalogParams)
            }
            "publish_catalog_params" => Ok(EntityType::PublishCatalogParams),
            "application/vnd.vmware.vcloud.query.queryList+xml" => Ok(EntityType::QueryList),
            "query_list" => Ok(EntityType::QueryList),
            "application/vnd.vmware.vcloud.rasdItem+xml" => Ok(EntityType::RasdItem),
            "rasd_item" => Ok(EntityType::RasdItem),
            "application/vnd.vmware.vcloud.rasdItemsList+xml" => Ok(EntityType::RasdItemsList),
            "rasd_items_list" => Ok(EntityType::RasdItemsList),
            "application/vnd.vmware.vcloud.recomposeVAppParams+xml" => {
                Ok(EntityType::RecomposeVappParams)
            }
            "recompose_vapp_params" => Ok(EntityType::RecomposeVappParams),
            "application/vnd.vmware.vcloud.query.records+xml" => Ok(EntityType::Records),
            "records" => Ok(EntityType::Records),
            "application/vnd.vmware.admin.registerVimServerParams+xml" => {
                Ok(EntityType::RegisterVcServerParams)
            }
            "register_vc_server_params" => Ok(EntityType::RegisterVcServerParams),
            "application/vnd.vmware.vcloud.relocateVmParams+xml" => Ok(EntityType::RelocateParams),
            "relocate_params" => Ok(EntityType::RelocateParams),
            "application/vnd.vmware.admin.resourcePoolList+xml" => Ok(EntityType::ResourcePoolList),
            "resource_pool_list" => Ok(EntityType::ResourcePoolList),
            "application/vnd.vmware.admin.resourcePoolSetUpdateParams+xml" => {
                Ok(EntityType::ResPoolSetUpdateParams)
            }
            "res_pool_set_update_params" => Ok(EntityType::ResPoolSetUpdateParams),
            "application/vnd.vmware.admin.role+xml" => Ok(EntityType::Role),
            "role" => Ok(EntityType::Role),
            "application/vnd.vmware.admin.right+xml" => Ok(EntityType::Right),
            "right" => Ok(EntityType::Right),
            "application/vnd.vmware.admin.rights+xml" => Ok(EntityType::Rights),
            "rights" => Ok(EntityType::Rights),
            "application/vnd.vmware.vcloud.createSnapshotParams+xml" => {
                Ok(EntityType::SnapshotCreate)
            }
            "snapshot_create" => Ok(EntityType::SnapshotCreate),
            "application/vnd.vmware.vcloud.startupSection+xml" => Ok(EntityType::StartupSection),
            "startup_section" => Ok(EntityType::StartupSection),
            "application/vnd.vmware.admin.systemSettings+xml" => Ok(EntityType::SystemSettings),
            "system_settings" => Ok(EntityType::SystemSettings),
            "application/vnd.vmware.vcloud.task+xml" => Ok(EntityType::Task),
            "task" => Ok(EntityType::Task),
            "application/vnd.vmware.vcloud.tasksList+xml" => Ok(EntityType::TasksList),
            "tasks_list" => Ok(EntityType::TasksList),
            "text/xml" => Ok(EntityType::TextXml),
            "text_xml" => Ok(EntityType::TextXml),
            "application/vnd.vmware.vcloud.undeployVAppParams+xml" => Ok(EntityType::Undeploy),
            "undeploy" => Ok(EntityType::Undeploy),
            "application/vnd.vmware.admin.updateProviderVdcStorageProfiles+xml" => {
                Ok(EntityType::UpdateProviderVdcStorageProfiles)
            }
            "update_provider_vdc_storage_profiles" => {
                Ok(EntityType::UpdateProviderVdcStorageProfiles)
            }
            "application/vnd.vmware.admin.updateVdcStorageProfiles+xml" => {
                Ok(EntityType::UpdateVdcStorageProfiles)
            }
            "update_vdc_storage_profiles" => Ok(EntityType::UpdateVdcStorageProfiles),
            "application/vnd.vmware.vcloud.uploadVAppTemplateParams+xml" => {
                Ok(EntityType::UploadVappTemplateParams)
            }
            "upload_vapp_template_params" => Ok(EntityType::UploadVappTemplateParams),
            "application/vnd.vmware.admin.user+xml" => Ok(EntityType::User),
            "user" => Ok(EntityType::User),
            "application/vnd.vmware.vcloud.vApp+xml" => Ok(EntityType::Vapp),
            "vapp" => Ok(EntityType::Vapp),
            "application/vnd.vmware.vcloud.vAppTemplate+xml" => Ok(EntityType::VappTemplate),
            "vapp_template" => Ok(EntityType::VappTemplate),
            "application/vnd.vmware.vcloud.vdc+xml" => Ok(EntityType::Vdc),
            "vdc" => Ok(EntityType::Vdc),
            "application/vnd.vmware.admin.vdc+xml" => Ok(EntityType::VdcAdmin),
            "vdc_admin" => Ok(EntityType::VdcAdmin),
            "application/vnd.vmware.vcloud.vdcComputePolicyReferences+xml" => {
                Ok(EntityType::VdcComputePolicyReferences)
            }
            "vdc_compute_policy_references" => Ok(EntityType::VdcComputePolicyReferences),
            "application/vnd.vmware.admin.vdcReferences+xml" => Ok(EntityType::VdcReferences),
            "vdc_references" => Ok(EntityType::VdcReferences),
            "application/vnd.vmware.vcloud.vdcStorageProfile+xml" => {
                Ok(EntityType::VdcStorageProfile)
            }
            "vdc_storage_profile" => Ok(EntityType::VdcStorageProfile),
            "application/vnd.vmware.admin.vdcStorageProfile+xml" => {
                Ok(EntityType::VdcStorageProfileAdmin)
            }
            "vdc_storage_profile_admin" => Ok(EntityType::VdcStorageProfileAdmin),
            "application/vnd.vmware.admin.createVdcParams+xml" => Ok(EntityType::VdcsParams),
            "vdcs_params" => Ok(EntityType::VdcsParams),
            "application/vnd.vmware.admin.vmwVimServerReferences+xml" => {
                Ok(EntityType::VimServerRefs)
            }
            "vim_server_refs" => Ok(EntityType::VimServerRefs),
            "application/vnd.vmware.admin.vmwvirtualcenter+xml" => Ok(EntityType::VirtualCenter),
            "virtual_center" => Ok(EntityType::VirtualCenter),
            "application/vnd.vmware.vcloud.vm+xml" => Ok(EntityType::Vm),
            "vm" => Ok(EntityType::Vm),
            "application/vnd.vmware.vcloud.bootOptionsSection+xml" => Ok(EntityType::VmBootOptions),
            "vm_boot_options" => Ok(EntityType::VmBootOptions),
            "application/vnd.vmware.vcloud.vmCapabilitiesSection+xml" => {
                Ok(EntityType::VmCapabilitiesSection)
            }
            "vm_capabilities_section" => Ok(EntityType::VmCapabilitiesSection),
            "application/vnd.vmware.vcloud.vms+xml" => Ok(EntityType::Vms),
            "vms" => Ok(EntityType::Vms),
            "application/vnd.vmware.vcloud.screenTicket+xml" => {
                Ok(EntityType::VmScreenAcquireTicket)
            }
            "vm_screen_acquire_ticket" => Ok(EntityType::VmScreenAcquireTicket),
            "application/vnd.vmware.vcloud.mksTicket+xml" => {
                Ok(EntityType::VmScreenAcquireMksticket)
            }
            "vm_screen_acquire_mksticket" => Ok(EntityType::VmScreenAcquireMksticket),
            "application/vnd.vmware.admin.vmwProviderVdcResourcePool+xml" => {
                Ok(EntityType::VmwProviderVdcResourcePool)
            }
            "vmw_provider_vdc_resource_pool" => Ok(EntityType::VmwProviderVdcResourcePool),
            "application/vnd.vmware.admin.vmwProviderVdcResourcePoolSet+xml" => {
                Ok(EntityType::VmwProviderVdcResourcePoolSet)
            }
            "vmw_provider_vdc_resource_pool_set" => Ok(EntityType::VmwProviderVdcResourcePoolSet),
            "application/vnd.vmware.admin.vmwPvdcStorageProfile+xml" => {
                Ok(EntityType::VmwPvdcStorageProfile)
            }
            "vmw_pvdc_storage_profile" => Ok(EntityType::VmwPvdcStorageProfile),
            "application/vnd.vmware.admin.vmwStorageProfiles+xml" => {
                Ok(EntityType::VmwStorageProfiles)
            }
            "vmw_storage_profiles" => Ok(EntityType::VmwStorageProfiles),
            "application/vnd.vmware.vcloud.vAppNetwork+xml" => Ok(EntityType::VappNetwork),
            "vapp_network" => Ok(EntityType::VappNetwork),
            _ => Err(()),
        }
    }
}

pub const ENTITY_TYPES: &[EntityType] = &[
    EntityType::Admin,
    EntityType::AdminCatalog,
    EntityType::AdminOrg,
    EntityType::AdminService,
    EntityType::AllocatedNetworkAddress,
    EntityType::AmqpSettings,
    EntityType::ApiExtensibility,
    EntityType::ApiFilter,
    EntityType::ApplicationBinary,
    EntityType::Catalog,
    EntityType::CaptureVappParams,
    EntityType::CheckPostGuestCustomizationSection,
    EntityType::CloneVappParams,
    EntityType::ComposeVappParams,
    EntityType::ComplianceResult,
    EntityType::ControlAccessParams,
    EntityType::CurrentUsage,
    EntityType::DatastoreReferences,
    EntityType::DefaultContentType,
    EntityType::Deploy,
    EntityType::Disk,
    EntityType::DiskAttachDetachParams,
    EntityType::DiskCreateParms,
    EntityType::EdgeGateway,
    EntityType::EdgeGatewayFormFactor,
    EntityType::EdgeGatewayServiceConfiguration,
    EntityType::EdgeGatewaySysLogServerIp,
    EntityType::Extension,
    EntityType::ExtensionServices,
    EntityType::ExternalNetwork,
    EntityType::ExternalNetworkRefs,
    EntityType::GuestCustomizationSection,
    EntityType::HistoricUsage,
    EntityType::Host,
    EntityType::HostRefs,
    EntityType::InstantiateVappTemplateParams,
    EntityType::Json,
    EntityType::LeaseSettings,
    EntityType::Media,
    EntityType::MediaInsertOrEjectParams,
    EntityType::Metadata,
    EntityType::MetadataValue,
    EntityType::NetworkConfigSection,
    EntityType::NetworkConnectionSection,
    EntityType::NetworkManagers,
    EntityType::NetworkPoolReferences,
    EntityType::NsxtManager,
    EntityType::OperatingSystemSection,
    EntityType::Org,
    EntityType::OrgNetwork,
    EntityType::OrgList,
    EntityType::OrgRights,
    EntityType::OrgVdcNetwork,
    EntityType::Owner,
    EntityType::ProductSections,
    EntityType::ProviderVdc,
    EntityType::ProviderVdcParams,
    EntityType::PublishCatalogParams,
    EntityType::QueryList,
    EntityType::RasdItem,
    EntityType::RasdItemsList,
    EntityType::RecomposeVappParams,
    EntityType::Records,
    EntityType::RegisterVcServerParams,
    EntityType::RelocateParams,
    EntityType::ResourcePoolList,
    EntityType::ResPoolSetUpdateParams,
    EntityType::Role,
    EntityType::Right,
    EntityType::Rights,
    EntityType::SnapshotCreate,
    EntityType::StartupSection,
    EntityType::SystemSettings,
    EntityType::Task,
    EntityType::TasksList,
    EntityType::TextXml,
    EntityType::Undeploy,
    EntityType::UpdateProviderVdcStorageProfiles,
    EntityType::UpdateVdcStorageProfiles,
    EntityType::UploadVappTemplateParams,
    EntityType::User,
    EntityType::Vapp,
    EntityType::VappTemplate,
    EntityType::Vdc,
    EntityType::VdcAdmin,
    EntityType::VdcComputePolicyReferences,
    EntityType::VdcReferences,
    EntityType::VdcStorageProfile,
    EntityType::VdcStorageProfileAdmin,
    EntityType::VdcsParams,
    EntityType::VimServerRefs,
    EntityType::VirtualCenter,
    EntityType::Vm,
    EntityType::VmBootOptions,
    EntityType::VmCapabilitiesSection,
    EntityType::Vms,
    EntityType::VmScreenAcquireTicket,
    EntityType::VmScreenAcquireMksticket,
    EntityType::VmwProviderVdcResourcePool,
    EntityType::VmwProviderVdcResourcePoolSet,
    EntityType::VmwPvdcStorageProfile,
    EntityType::VmwStorageProfiles,
    EntityType::VappNetwork,
];

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

/// Domains used for metadata entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataDomain {
    General,
    System,
}

impl std::fmt::Display for MetadataDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MetadataDomain::General => "GENERAL",
            MetadataDomain::System => "SYSTEM",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for MetadataDomain {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GENERAL" => Ok(MetadataDomain::General),
            "SYSTEM" => Ok(MetadataDomain::System),
            _ => Err(()),
        }
    }
}

/// Visibility levels for metadata keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataVisibility {
    Private,
    ReadOnly,
    ReadWrite,
}

impl std::fmt::Display for MetadataVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MetadataVisibility::Private => "PRIVATE",
            MetadataVisibility::ReadOnly => "READONLY",
            MetadataVisibility::ReadWrite => "READWRITE",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for MetadataVisibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PRIVATE" => Ok(MetadataVisibility::Private),
            "READONLY" => Ok(MetadataVisibility::ReadOnly),
            "READWRITE" => Ok(MetadataVisibility::ReadWrite),
            _ => Err(()),
        }
    }
}

/// Possible data types for metadata values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataValueType {
    String,
    Number,
    Boolean,
    DateTime,
}

impl std::fmt::Display for MetadataValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MetadataValueType::String => "MetadataStringValue",
            MetadataValueType::Number => "MetadataNumberValue",
            MetadataValueType::Boolean => "MetadataBooleanValue",
            MetadataValueType::DateTime => "MetadataDateTimeValue",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for MetadataValueType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MetadataStringValue" => Ok(MetadataValueType::String),
            "MetadataNumberValue" => Ok(MetadataValueType::Number),
            "MetadataBooleanValue" => Ok(MetadataValueType::Boolean),
            "MetadataDateTimeValue" => Ok(MetadataValueType::DateTime),
            _ => Err(()),
        }
    }
}

/// Status values for asynchronous tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Queued,
    PreRunning,
    Running,
    Success,
    Error,
    Canceled,
    Aborted,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TaskStatus::Queued => "queued",
            TaskStatus::PreRunning => "preRunning",
            TaskStatus::Running => "running",
            TaskStatus::Success => "success",
            TaskStatus::Error => "error",
            TaskStatus::Canceled => "canceled",
            TaskStatus::Aborted => "aborted",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "queued" => Ok(TaskStatus::Queued),
            "preRunning" => Ok(TaskStatus::PreRunning),
            "running" => Ok(TaskStatus::Running),
            "success" => Ok(TaskStatus::Success),
            "error" => Ok(TaskStatus::Error),
            "canceled" => Ok(TaskStatus::Canceled),
            "aborted" => Ok(TaskStatus::Aborted),
            _ => Err(()),
        }
    }
}

/// Edge gateway form factor options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayBackingConfigType {
    Compact,
    Full,
    Full4,
    XLarge,
}

impl std::fmt::Display for GatewayBackingConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GatewayBackingConfigType::Compact => "compact",
            GatewayBackingConfigType::Full => "full",
            GatewayBackingConfigType::Full4 => "full4",
            GatewayBackingConfigType::XLarge => "x-large",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for GatewayBackingConfigType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "compact" => Ok(GatewayBackingConfigType::Compact),
            "full" => Ok(GatewayBackingConfigType::Full),
            "full4" => Ok(GatewayBackingConfigType::Full4),
            "x-large" => Ok(GatewayBackingConfigType::XLarge),
            _ => Err(()),
        }
    }
}

/// Power state values reported for vApps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VAppPowerStatus {
    Running,
    Stopped,
    Suspended,
    Deployed,
    Undeployed,
}

impl std::fmt::Display for VAppPowerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            VAppPowerStatus::Running => "4",
            VAppPowerStatus::Stopped => "8",
            VAppPowerStatus::Suspended => "3",
            VAppPowerStatus::Deployed => "2",
            VAppPowerStatus::Undeployed => "1",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for VAppPowerStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "4" => Ok(VAppPowerStatus::Running),
            "8" => Ok(VAppPowerStatus::Stopped),
            "3" => Ok(VAppPowerStatus::Suspended),
            "2" => Ok(VAppPowerStatus::Deployed),
            "1" => Ok(VAppPowerStatus::Undeployed),
            _ => Err(()),
        }
    }
}

/// General status values returned by vCloud Director objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum VCloudStatus {
    CouldNotBeCreated = -1,
    Unresolved = 0,
    Resolved = 1,
    Deployed = 2,
    Suspended = 3,
    PoweredOn = 4,
    WaitingForUserInput = 5,
    UnknownState = 6,
    UnrecognizedState = 7,
    PoweredOff = 8,
    InconsistentState = 9,
    ChildrenDoNotAllHaveSameStatus = 10,
    UploadInitiatedOvfDescriptorPending = 11,
    UploadInitiatedCopyingContents = 12,
    UploadInitiatedDiskContentsPending = 13,
    UploadQuarantined = 14,
    UploadQuarantineExpired = 15,
}

impl VCloudStatus {
    /// Return the textual description of the status code.
    pub fn description(&self) -> &'static str {
        match self {
            VCloudStatus::CouldNotBeCreated => "Could not be created",
            VCloudStatus::Unresolved => "Unresolved",
            VCloudStatus::Resolved => "Resolved",
            VCloudStatus::Deployed => "Deployed",
            VCloudStatus::Suspended => "Suspended",
            VCloudStatus::PoweredOn => "Powered on",
            VCloudStatus::WaitingForUserInput => "Waiting for user input",
            VCloudStatus::UnknownState => "Unknown state",
            VCloudStatus::UnrecognizedState => "Unrecognized state",
            VCloudStatus::PoweredOff => "Powered off",
            VCloudStatus::InconsistentState => "Inconsistent state",
            VCloudStatus::ChildrenDoNotAllHaveSameStatus =>
                "Children do not all have the same status",
            VCloudStatus::UploadInitiatedOvfDescriptorPending =>
                "Upload initiated, OVF descriptor pending",
            VCloudStatus::UploadInitiatedCopyingContents =>
                "Upload initiated, copying contents",
            VCloudStatus::UploadInitiatedDiskContentsPending =>
                "Upload initiated , disk contents pending",
            VCloudStatus::UploadQuarantined => "Upload has been quarantined",
            VCloudStatus::UploadQuarantineExpired =>
                "Upload quarantine period has expired",
        }
    }

    /// Convert a numeric status code to a `VCloudStatus` value.
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            -1 => Some(VCloudStatus::CouldNotBeCreated),
            0 => Some(VCloudStatus::Unresolved),
            1 => Some(VCloudStatus::Resolved),
            2 => Some(VCloudStatus::Deployed),
            3 => Some(VCloudStatus::Suspended),
            4 => Some(VCloudStatus::PoweredOn),
            5 => Some(VCloudStatus::WaitingForUserInput),
            6 => Some(VCloudStatus::UnknownState),
            7 => Some(VCloudStatus::UnrecognizedState),
            8 => Some(VCloudStatus::PoweredOff),
            9 => Some(VCloudStatus::InconsistentState),
            10 => Some(VCloudStatus::ChildrenDoNotAllHaveSameStatus),
            11 => Some(VCloudStatus::UploadInitiatedOvfDescriptorPending),
            12 => Some(VCloudStatus::UploadInitiatedCopyingContents),
            13 => Some(VCloudStatus::UploadInitiatedDiskContentsPending),
            14 => Some(VCloudStatus::UploadQuarantined),
            15 => Some(VCloudStatus::UploadQuarantineExpired),
            _ => None,
        }
    }
}

/// Supported network fence modes for vApp networks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceMode {
    Isolated,
    Direct,
    Bridged,
    NatRouted,
}

impl std::fmt::Display for FenceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FenceMode::Isolated => "isolated",
            FenceMode::Direct => "direct",
            FenceMode::Bridged => "bridged",
            FenceMode::NatRouted => "natRouted",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for FenceMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "isolated" => Ok(FenceMode::Isolated),
            "direct" => Ok(FenceMode::Direct),
            "bridged" => Ok(FenceMode::Bridged),
            "natRouted" => Ok(FenceMode::NatRouted),
            _ => Err(()),
        }
    }
}

/// Types of logical network links.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalNetworkLinkType {
    Bridged,
    Independent,
    DlrUplink,
}

impl std::fmt::Display for LogicalNetworkLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LogicalNetworkLinkType::Bridged => "0",
            LogicalNetworkLinkType::Independent => "1",
            LogicalNetworkLinkType::DlrUplink => "2",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for LogicalNetworkLinkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(LogicalNetworkLinkType::Bridged),
            "1" => Ok(LogicalNetworkLinkType::Independent),
            "2" => Ok(LogicalNetworkLinkType::DlrUplink),
            _ => Err(()),
        }
    }
}

/// Virtual network adapter models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkAdapterType {
    Vmxnet,
    Vmxnet2,
    Vmxnet3,
    E1000,
    E1000e,
    Vlance,
}

impl std::fmt::Display for NetworkAdapterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NetworkAdapterType::Vmxnet => "VMXNET",
            NetworkAdapterType::Vmxnet2 => "VMXNET2",
            NetworkAdapterType::Vmxnet3 => "VMXNET3",
            NetworkAdapterType::E1000 => "E1000",
            NetworkAdapterType::E1000e => "E1000E",
            NetworkAdapterType::Vlance => "PCNet32",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for NetworkAdapterType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VMXNET" => Ok(NetworkAdapterType::Vmxnet),
            "VMXNET2" => Ok(NetworkAdapterType::Vmxnet2),
            "VMXNET3" => Ok(NetworkAdapterType::Vmxnet3),
            "E1000" => Ok(NetworkAdapterType::E1000),
            "E1000E" => Ok(NetworkAdapterType::E1000e),
            "PCNet32" => Ok(NetworkAdapterType::Vlance),
            _ => Err(()),
        }
    }
}

/// Standard rel values used in vCloud Director links.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationType {
    Add,
    Alternate,
    CheckCompliance,
    Consolidate,
    ControlAccess,
    ConvertToAdvancedGateway,
    CustomizeAtNextPowerOn,
    Deploy,
    Disable,
    DisableGatewayDistributedRouting,
    DiscardSuspendedState,
    DiskAttach,
    DiskDetach,
    Down,
    DownExtensibility,
    DownloadDefault,
    DownloadOvaDefault,
    EdgeGateways,
    Edit,
    EjectMedia,
    Enable,
    EnableGatewayDistributedRouting,
    EnterMaintenanceMode,
    ExitMaintenanceMode,
    GatewayRedeploy,
    GatewaySyncSyslogSettings,
    GatewaySysServerSettingIp,
    GatewayUpdateProperties,
    GuestCustomizationSection,
    InsertMedia,
    InstallVmwareTools,
    LinkToTemplate,
    Metrics,
    MigrateVms,
    ModifyFormFactor,
    NextPage,
    OpenApi,
    OrgVdcNetworks,
    PowerOff,
    PowerOn,
    PowerReboot,
    PowerReset,
    PowerShutdown,
    PowerSuspend,
    Publish,
    Recompose,
    ReconfigureVm,
    ReloadFromVc,
    Relocate,
    Remove,
    Repair,
    Rights,
    ResourcePoolVmList,
    SnapshotCreate,
    SnapshotRevertToCurrent,
    SnapshotRemoveAll,
    SyncSyslogSettings,
    TaskCancel,
    Undeploy,
    UnlinkFromTemplate,
    Unregister,
    Up,
    UpdateResourcePools,
    Upgrade,
    VdcRoutedConvertToDistributedInterface,
    VdcRoutedConvertToSubInterface,
    VdcRoutedConvertToInternalInterface,
}

impl std::fmt::Display for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RelationType::Add => "add",
            RelationType::Alternate => "alternate",
            RelationType::CheckCompliance => "checkCompliance",
            RelationType::Consolidate => "consolidate",
            RelationType::ControlAccess => "controlAccess",
            RelationType::ConvertToAdvancedGateway => "edgeGateway:convertToAdvancedGateway",
            RelationType::CustomizeAtNextPowerOn => "customizeAtNextPowerOn",
            RelationType::Deploy => "deploy",
            RelationType::Disable => "disable",
            RelationType::DisableGatewayDistributedRouting => {
                "edgeGateway:disableDistributedRouting"
            }
            RelationType::DiscardSuspendedState => "discardState",
            RelationType::DiskAttach => "disk:attach",
            RelationType::DiskDetach => "disk:detach",
            RelationType::Down => "down",
            RelationType::DownExtensibility => "down:extensibility",
            RelationType::DownloadDefault => "download:default",
            RelationType::DownloadOvaDefault => "download:ovaDefault",
            RelationType::EdgeGateways => "edgeGateways",
            RelationType::Edit => "edit",
            RelationType::EjectMedia => "media:ejectMedia",
            RelationType::Enable => "enable",
            RelationType::EnableGatewayDistributedRouting => "edgeGateway:enableDistributedRouting",
            RelationType::EnterMaintenanceMode => "enterMaintenanceMode",
            RelationType::ExitMaintenanceMode => "exitMaintenanceMode",
            RelationType::GatewayRedeploy => "edgeGateway:redeploy",
            RelationType::GatewaySyncSyslogSettings => "edgeGateway:syncSyslogSettings",
            RelationType::GatewaySysServerSettingIp => "edgeGateway:configureSyslogServerSettings",
            RelationType::GatewayUpdateProperties => "edgeGateway:updateProperties",
            RelationType::GuestCustomizationSection => "guestCustomizationSection",
            RelationType::InsertMedia => "media:insertMedia",
            RelationType::InstallVmwareTools => "installVmwareTools",
            RelationType::LinkToTemplate => "linkToTemplate",
            RelationType::Metrics => "metrics",
            RelationType::MigrateVms => "migrateVms",
            RelationType::ModifyFormFactor => "edgeGateway:modifyFormFactor",
            RelationType::NextPage => "nextPage",
            RelationType::OpenApi => "openapi",
            RelationType::OrgVdcNetworks => "orgVdcNetworks",
            RelationType::PowerOff => "power:powerOff",
            RelationType::PowerOn => "power:powerOn",
            RelationType::PowerReboot => "power:reboot",
            RelationType::PowerReset => "power:reset",
            RelationType::PowerShutdown => "power:shutdown",
            RelationType::PowerSuspend => "power:suspend",
            RelationType::Publish => "publish",
            RelationType::Recompose => "recompose",
            RelationType::ReconfigureVm => "reconfigureVm",
            RelationType::ReloadFromVc => "reloadFromVc",
            RelationType::Relocate => "relocate",
            RelationType::Remove => "remove",
            RelationType::Repair => "repair",
            RelationType::Rights => "rights",
            RelationType::ResourcePoolVmList => "resourcePoolVmList",
            RelationType::SnapshotCreate => "snapshot:create",
            RelationType::SnapshotRevertToCurrent => "snapshot:revertToCurrent",
            RelationType::SnapshotRemoveAll => "snapshot:removeAll",
            RelationType::SyncSyslogSettings => "syncSyslogSettings",
            RelationType::TaskCancel => "task:cancel",
            RelationType::Undeploy => "undeploy",
            RelationType::UnlinkFromTemplate => "unlinkFromTemplate",
            RelationType::Unregister => "unregister",
            RelationType::Up => "up",
            RelationType::UpdateResourcePools => "update:resourcePools",
            RelationType::Upgrade => "upgrade",
            RelationType::VdcRoutedConvertToDistributedInterface => {
                "orgVdcNetwork:convertToDistributedInterface"
            }
            RelationType::VdcRoutedConvertToSubInterface => "orgVdcNetwork:convertToSubInterface",
            RelationType::VdcRoutedConvertToInternalInterface => {
                "orgVdcNetwork:convertToInternalInterface"
            }
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for RelationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(RelationType::Add),
            "alternate" => Ok(RelationType::Alternate),
            "checkCompliance" => Ok(RelationType::CheckCompliance),
            "consolidate" => Ok(RelationType::Consolidate),
            "controlAccess" => Ok(RelationType::ControlAccess),
            "edgeGateway:convertToAdvancedGateway" => Ok(RelationType::ConvertToAdvancedGateway),
            "customizeAtNextPowerOn" => Ok(RelationType::CustomizeAtNextPowerOn),
            "deploy" => Ok(RelationType::Deploy),
            "disable" => Ok(RelationType::Disable),
            "edgeGateway:disableDistributedRouting" => {
                Ok(RelationType::DisableGatewayDistributedRouting)
            }
            "discardState" => Ok(RelationType::DiscardSuspendedState),
            "disk:attach" => Ok(RelationType::DiskAttach),
            "disk:detach" => Ok(RelationType::DiskDetach),
            "down" => Ok(RelationType::Down),
            "down:extensibility" => Ok(RelationType::DownExtensibility),
            "download:default" => Ok(RelationType::DownloadDefault),
            "download:ovaDefault" => Ok(RelationType::DownloadOvaDefault),
            "edgeGateways" => Ok(RelationType::EdgeGateways),
            "edit" => Ok(RelationType::Edit),
            "media:ejectMedia" => Ok(RelationType::EjectMedia),
            "enable" => Ok(RelationType::Enable),
            "edgeGateway:enableDistributedRouting" => {
                Ok(RelationType::EnableGatewayDistributedRouting)
            }
            "enterMaintenanceMode" => Ok(RelationType::EnterMaintenanceMode),
            "exitMaintenanceMode" => Ok(RelationType::ExitMaintenanceMode),
            "edgeGateway:redeploy" => Ok(RelationType::GatewayRedeploy),
            "edgeGateway:syncSyslogSettings" => Ok(RelationType::GatewaySyncSyslogSettings),
            "edgeGateway:configureSyslogServerSettings" => {
                Ok(RelationType::GatewaySysServerSettingIp)
            }
            "edgeGateway:updateProperties" => Ok(RelationType::GatewayUpdateProperties),
            "guestCustomizationSection" => Ok(RelationType::GuestCustomizationSection),
            "media:insertMedia" => Ok(RelationType::InsertMedia),
            "installVmwareTools" => Ok(RelationType::InstallVmwareTools),
            "linkToTemplate" => Ok(RelationType::LinkToTemplate),
            "metrics" => Ok(RelationType::Metrics),
            "migrateVms" => Ok(RelationType::MigrateVms),
            "edgeGateway:modifyFormFactor" => Ok(RelationType::ModifyFormFactor),
            "nextPage" => Ok(RelationType::NextPage),
            "openapi" => Ok(RelationType::OpenApi),
            "orgVdcNetworks" => Ok(RelationType::OrgVdcNetworks),
            "power:powerOff" => Ok(RelationType::PowerOff),
            "power:powerOn" => Ok(RelationType::PowerOn),
            "power:reboot" => Ok(RelationType::PowerReboot),
            "power:reset" => Ok(RelationType::PowerReset),
            "power:shutdown" => Ok(RelationType::PowerShutdown),
            "power:suspend" => Ok(RelationType::PowerSuspend),
            "publish" => Ok(RelationType::Publish),
            "recompose" => Ok(RelationType::Recompose),
            "reconfigureVm" => Ok(RelationType::ReconfigureVm),
            "reloadFromVc" => Ok(RelationType::ReloadFromVc),
            "relocate" => Ok(RelationType::Relocate),
            "remove" => Ok(RelationType::Remove),
            "repair" => Ok(RelationType::Repair),
            "rights" => Ok(RelationType::Rights),
            "resourcePoolVmList" => Ok(RelationType::ResourcePoolVmList),
            "snapshot:create" => Ok(RelationType::SnapshotCreate),
            "snapshot:revertToCurrent" => Ok(RelationType::SnapshotRevertToCurrent),
            "snapshot:removeAll" => Ok(RelationType::SnapshotRemoveAll),
            "syncSyslogSettings" => Ok(RelationType::SyncSyslogSettings),
            "task:cancel" => Ok(RelationType::TaskCancel),
            "undeploy" => Ok(RelationType::Undeploy),
            "unlinkFromTemplate" => Ok(RelationType::UnlinkFromTemplate),
            "unregister" => Ok(RelationType::Unregister),
            "up" => Ok(RelationType::Up),
            "update:resourcePools" => Ok(RelationType::UpdateResourcePools),
            "upgrade" => Ok(RelationType::Upgrade),
            "orgVdcNetwork:convertToDistributedInterface" => {
                Ok(RelationType::VdcRoutedConvertToDistributedInterface)
            }
            "orgVdcNetwork:convertToSubInterface" => {
                Ok(RelationType::VdcRoutedConvertToSubInterface)
            }
            "orgVdcNetwork:convertToInternalInterface" => {
                Ok(RelationType::VdcRoutedConvertToInternalInterface)
            }
            _ => Err(()),
        }
    }
}

/// Formats for query results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryResultFormat {
    Records,
    IdRecords,
    References,
}

impl QueryResultFormat {
    pub fn content_type(&self) -> &'static str {
        match self {
            QueryResultFormat::Records => "application/vnd.vmware.vcloud.query.records+xml",
            QueryResultFormat::IdRecords => "application/vnd.vmware.vcloud.query.idrecords+xml",
            QueryResultFormat::References => "application/vnd.vmware.vcloud.query.references+xml",
        }
    }

    pub fn alias(&self) -> &'static str {
        match self {
            QueryResultFormat::Records => "records",
            QueryResultFormat::IdRecords => "idrecords",
            QueryResultFormat::References => "references",
        }
    }
}

impl std::fmt::Display for QueryResultFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.alias())
    }
}

impl std::str::FromStr for QueryResultFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "records" | "application/vnd.vmware.vcloud.query.records+xml" => {
                Ok(QueryResultFormat::Records)
            }
            "idrecords" | "application/vnd.vmware.vcloud.query.idrecords+xml" => {
                Ok(QueryResultFormat::IdRecords)
            }
            "references" | "application/vnd.vmware.vcloud.query.references+xml" => {
                Ok(QueryResultFormat::References)
            }
            _ => Err(()),
        }
    }
}

/// Well known endpoints available from a Session resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WellKnownEndpoint {
    LoggedInOrg,
    OrgVdc,
    OrgNetwork,
    OrgCatalog,
    QueryList,
    Admin,
    ApiExtensibility,
    Extension,
    OrgList,
    SnapshotCreate,
    OpenApi,
}

impl WellKnownEndpoint {
    pub fn relation(&self) -> RelationType {
        match self {
            WellKnownEndpoint::LoggedInOrg => RelationType::Down,
            WellKnownEndpoint::OrgVdc => RelationType::Down,
            WellKnownEndpoint::OrgNetwork => RelationType::Down,
            WellKnownEndpoint::OrgCatalog => RelationType::Down,
            WellKnownEndpoint::QueryList => RelationType::Down,
            WellKnownEndpoint::Admin => RelationType::Down,
            WellKnownEndpoint::ApiExtensibility => RelationType::DownExtensibility,
            WellKnownEndpoint::Extension => RelationType::Down,
            WellKnownEndpoint::OrgList => RelationType::Down,
            WellKnownEndpoint::SnapshotCreate => RelationType::SnapshotCreate,
            WellKnownEndpoint::OpenApi => RelationType::OpenApi,
        }
    }

    pub fn entity(&self) -> EntityType {
        match self {
            WellKnownEndpoint::LoggedInOrg => EntityType::Org,
            WellKnownEndpoint::OrgVdc => EntityType::Vdc,
            WellKnownEndpoint::OrgNetwork => EntityType::OrgNetwork,
            WellKnownEndpoint::OrgCatalog => EntityType::Catalog,
            WellKnownEndpoint::QueryList => EntityType::QueryList,
            WellKnownEndpoint::Admin => EntityType::Admin,
            WellKnownEndpoint::ApiExtensibility => EntityType::ApiExtensibility,
            WellKnownEndpoint::Extension => EntityType::Extension,
            WellKnownEndpoint::OrgList => EntityType::OrgList,
            WellKnownEndpoint::SnapshotCreate => EntityType::SnapshotCreate,
            WellKnownEndpoint::OpenApi => EntityType::Json,
        }
    }
}

pub const WELL_KNOWN_ENDPOINTS: &[WellKnownEndpoint] = &[
    WellKnownEndpoint::LoggedInOrg,
    WellKnownEndpoint::OrgVdc,
    WellKnownEndpoint::OrgNetwork,
    WellKnownEndpoint::OrgCatalog,
    WellKnownEndpoint::QueryList,
    WellKnownEndpoint::Admin,
    WellKnownEndpoint::ApiExtensibility,
    WellKnownEndpoint::Extension,
    WellKnownEndpoint::OrgList,
    WellKnownEndpoint::SnapshotCreate,
    WellKnownEndpoint::OpenApi,
];
/// Return the vCloud status message for a given status code.
pub fn vcloud_status_message(status: i32) -> Option<&'static str> {
    VCloudStatus::from_code(status).map(|s| s.description())
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

    #[test]
    fn vcloud_status_roundtrip() {
        let status = VCloudStatus::from_code(8).unwrap();
        assert_eq!(status, VCloudStatus::PoweredOff);
        assert_eq!(status.description(), "Powered off");
    }

    #[test]
    fn parse_metadata_domain() {
        let d: MetadataDomain = "GENERAL".parse().unwrap();
        assert_eq!(d, MetadataDomain::General);
        assert_eq!(d.to_string(), "GENERAL");
    }

    #[test]
    fn parse_task_status() {
        let s: TaskStatus = "running".parse().unwrap();
        assert_eq!(s, TaskStatus::Running);
        assert_eq!(s.to_string(), "running");
    }

    #[test]
    fn parse_vapp_power_status() {
        let p: VAppPowerStatus = "4".parse().unwrap();
        assert_eq!(p, VAppPowerStatus::Running);
        assert_eq!(p.to_string(), "4");
    }

    #[test]
    fn parse_fence_mode() {
        let f: FenceMode = "bridged".parse().unwrap();
        assert_eq!(f, FenceMode::Bridged);
        assert_eq!(f.to_string(), "bridged");
    }

    #[test]
    fn parse_logical_network_link_type() {
        let t: LogicalNetworkLinkType = "1".parse().unwrap();
        assert_eq!(t, LogicalNetworkLinkType::Independent);
        assert_eq!(t.to_string(), "1");
    }

    #[test]
    fn parse_network_adapter_type() {
        let t: NetworkAdapterType = "VMXNET3".parse().unwrap();
        assert_eq!(t, NetworkAdapterType::Vmxnet3);
        assert_eq!(t.to_string(), "VMXNET3");
    }

    #[test]
    fn parse_relation_type() {
        let r: RelationType = "power:powerOff".parse().unwrap();
        assert_eq!(r, RelationType::PowerOff);
        assert_eq!(r.to_string(), "power:powerOff");
    }

    #[test]
    fn parse_query_result_format() {
        let f: QueryResultFormat = "idrecords".parse().unwrap();
        assert_eq!(f, QueryResultFormat::IdRecords);
        assert_eq!(
            f.content_type(),
            "application/vnd.vmware.vcloud.query.idrecords+xml"
        );
    }

    #[test]
    fn parse_resource_type() {
        let r: ResourceType = "catalog".parse().unwrap();
        assert_eq!(r, ResourceType::Catalog);
        assert_eq!(r.to_string(), "catalog");
    }

    #[test]
    fn resource_types_len() {
        assert_eq!(RESOURCE_TYPES.len(), 79);
    }

    #[test]
    fn parse_entity_type() {
        let e: EntityType = "application/vnd.vmware.vcloud.vApp+xml".parse().unwrap();
        assert_eq!(e, EntityType::Vapp);
        assert_eq!(e.to_string(), "application/vnd.vmware.vcloud.vApp+xml");
    }

    #[test]
    fn entity_types_len() {
        assert_eq!(ENTITY_TYPES.len(), 103);
    }
    #[test]
    fn well_known_endpoint_mapping() {
        assert_eq!(WellKnownEndpoint::OpenApi.relation(), RelationType::OpenApi);
        assert_eq!(WellKnownEndpoint::OpenApi.entity(), EntityType::Json);
    }

    #[test]
    fn well_known_endpoints_len() {
        assert_eq!(WELL_KNOWN_ENDPOINTS.len(), 11);
    }
}
