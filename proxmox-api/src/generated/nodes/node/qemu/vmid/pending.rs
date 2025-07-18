#[derive(Debug, Clone)]
pub struct PendingClient<T> {
    client: T,
    path: String,
}
impl<T> PendingClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/pending"),
        }
    }
}
impl<T> PendingClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the virtual machine configuration with both current and pending values."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(key: String) -> Self {
        Self {
            key,
            delete: Default::default(),
            pending: Default::default(),
            value: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Indicates a pending delete request if present and not 0. The value 2 indicates a force-delete request."]
    #[doc = ""]
    pub delete: Option<i64>,
    #[doc = "Configuration option name."]
    #[doc = ""]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Pending value."]
    #[doc = ""]
    pub pending: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Current value."]
    #[doc = ""]
    pub value: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
