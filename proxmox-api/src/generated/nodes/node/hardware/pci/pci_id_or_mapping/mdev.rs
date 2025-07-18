#[derive(Debug, Clone)]
pub struct MdevClient<T> {
    client: T,
    path: String,
}
impl<T> MdevClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/mdev"),
        }
    }
}
impl<T> MdevClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List mediated device types for given PCI device."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(available: i64, description: String, ty: String) -> Self {
        Self {
            available,
            description,
            ty,
            name: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The number of still available instances of this type."]
    #[doc = ""]
    pub available: i64,
    #[doc = "Additional description of the type."]
    #[doc = ""]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A human readable name for the type."]
    #[doc = ""]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[doc = "The name of the mdev type."]
    #[doc = ""]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
