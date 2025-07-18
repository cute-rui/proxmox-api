#[derive(Debug, Clone)]
pub struct ExtractconfigClient<T> {
    client: T,
    path: String,
}
impl<T> ExtractconfigClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/extractconfig"),
        }
    }
}
impl<T> ExtractconfigClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Extract configuration from vzdump backup archive."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetParams {
    pub fn new(volume: String) -> Self {
        Self {
            volume,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "Volume identifier"]
    #[doc = ""]
    pub volume: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
