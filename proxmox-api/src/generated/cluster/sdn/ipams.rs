pub mod ipam;
#[derive(Debug, Clone)]
pub struct IpamsClient<T> {
    client: T,
    path: String,
}
impl<T> IpamsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/ipams"),
        }
    }
}
impl<T> IpamsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN ipams index."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> IpamsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new sdn ipam object."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(ipam: String, ty: String) -> Self {
        Self {
            ipam,
            ty,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub ipam: String,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list sdn ipams of specific type"]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(ipam: String, ty: Type) -> Self {
        Self {
            ipam,
            ty,
            fingerprint: Default::default(),
            section: Default::default(),
            token: Default::default(),
            url: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate SHA 256 fingerprint."]
    #[doc = ""]
    pub fingerprint: Option<String>,
    #[doc = "The SDN ipam object identifier."]
    #[doc = ""]
    pub ipam: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub section: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Plugin type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Only list sdn ipams of specific type"]
#[doc = ""]
pub enum Type {
    #[serde(rename = "netbox")]
    Netbox,
    #[serde(rename = "phpipam")]
    Phpipam,
    #[serde(rename = "pve")]
    Pve,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "netbox" => Ok(Self::Netbox),
            "phpipam" => Ok(Self::Phpipam),
            "pve" => Ok(Self::Pve),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> IpamsClient<T>
where
    T: crate::client::Client,
{
    pub fn ipam(&self, ipam: &str) -> ipam::IpamClient<T> {
        ipam::IpamClient::<T>::new(self.client.clone(), &self.path, ipam)
    }
}
