pub mod export;
pub mod server;
#[derive(Debug, Clone)]
pub struct MetricsClient<T> {
    client: T,
    path: String,
}
impl<T> MetricsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/metrics"),
        }
    }
}
impl<T> MetricsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Metrics index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> MetricsClient<T>
where
    T: crate::client::Client,
{
    pub fn server(&self) -> server::ServerClient<T> {
        server::ServerClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> MetricsClient<T>
where
    T: crate::client::Client,
{
    pub fn export(&self) -> export::ExportClient<T> {
        export::ExportClient::<T>::new(self.client.clone(), &self.path)
    }
}
