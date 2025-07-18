#[derive(Debug, Clone)]
pub struct MigrateClient<T> {
    client: T,
    path: String,
}
impl<T> MigrateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/migrate"),
        }
    }
}
impl<T> MigrateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get preconditions for migration."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> MigrateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Migrate virtual machine. Creates a new migration task."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutput {
    pub fn new(
        local_disks: Vec<LocalDisksGetOutputLocalDisksItems>,
        local_resources: Vec<String>,
        mapped_resource_info: MappedResourceInfoGetOutputMappedResourceInfo,
        mapped_resources: Vec<String>,
        running: bool,
    ) -> Self {
        Self {
            local_disks,
            local_resources,
            mapped_resource_info,
            mapped_resources,
            running,
            allowed_nodes: Default::default(),
            not_allowed_nodes: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of nodes allowed for migration."]
    #[doc = ""]
    pub allowed_nodes: Vec<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List local disks including CD-Rom, unused and not referenced disks"]
    #[doc = ""]
    pub local_disks: Vec<LocalDisksGetOutputLocalDisksItems>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List local resources (e.g. pci, usb) that block migration."]
    #[doc = ""]
    pub local_resources: Vec<String>,
    #[serde(rename = "mapped-resource-info")]
    #[doc = "Object of mapped resources with additional information such if they're live migratable."]
    #[doc = ""]
    pub mapped_resource_info: MappedResourceInfoGetOutputMappedResourceInfo,
    #[serde(rename = "mapped-resources")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of mapped resources e.g. pci, usb. Deprecated, use 'mapped-resource-info' instead."]
    #[doc = ""]
    pub mapped_resources: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of not allowed nodes with additional information."]
    #[doc = ""]
    pub not_allowed_nodes: Option<NotAllowedNodesGetOutputNotAllowedNodes>,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "Determines if the VM is running."]
    #[doc = ""]
    pub running: bool,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target node."]
    #[doc = ""]
    pub target: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl LocalDisksGetOutputLocalDisksItems {
    pub fn new(cdrom: bool, is_unused: bool, size: i64, volid: String) -> Self {
        Self {
            cdrom,
            is_unused,
            size,
            volid,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct LocalDisksGetOutputLocalDisksItems {
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "True if the disk is a cdrom."]
    #[doc = ""]
    pub cdrom: bool,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "True if the disk is unused."]
    #[doc = ""]
    pub is_unused: bool,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The size of the disk in bytes."]
    #[doc = ""]
    pub size: i64,
    #[doc = "The volid of the disk."]
    #[doc = ""]
    pub volid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct MappedResourceInfoGetOutputMappedResourceInfo {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct NotAllowedNodesGetOutputNotAllowedNodes {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of not available storages."]
    #[doc = ""]
    pub unavailable_storages: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(target: String) -> Self {
        Self {
            target,
            bwlimit: Default::default(),
            force: Default::default(),
            migration_network: Default::default(),
            migration_type: Default::default(),
            online: Default::default(),
            targetstorage: Default::default(),
            with_local_disks: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow to migrate VMs which use local devices. Only root may use this option."]
    #[doc = ""]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CIDR of the (sub) network that is used for migration."]
    #[doc = ""]
    pub migration_network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Migration traffic is encrypted using an SSH tunnel by default. On secure, completely private networks this can be disabled to increase performance."]
    #[doc = ""]
    pub migration_type: Option<MigrationType>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use online/live migration if VM is running. Ignored if VM is stopped."]
    #[doc = ""]
    pub online: Option<bool>,
    #[doc = "Target node."]
    #[doc = ""]
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    #[doc = ""]
    pub targetstorage: Option<String>,
    #[serde(rename = "with-local-disks")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable live storage migration for local disk"]
    #[doc = ""]
    pub with_local_disks: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Migration traffic is encrypted using an SSH tunnel by default. On secure, completely private networks this can be disabled to increase performance."]
#[doc = ""]
pub enum MigrationType {
    #[serde(rename = "insecure")]
    Insecure,
    #[serde(rename = "secure")]
    Secure,
}
impl TryFrom<&str> for MigrationType {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "insecure" => Ok(Self::Insecure),
            "secure" => Ok(Self::Secure),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
