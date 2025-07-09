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
}
