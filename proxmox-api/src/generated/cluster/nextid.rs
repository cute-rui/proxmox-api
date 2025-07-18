#[derive(Debug, Clone)]
pub struct NextidClient<T> {
    client: T,
    path: String,
}
impl<T> NextidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/nextid"),
        }
    }
}
impl<T> NextidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get next free VMID. Pass a VMID to assert that its free (at time of check)."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<i64, T::Error> {
        let path = self.path.to_string();
        Ok(self
            .client
            .get::<_, crate::types::Integer>(&path, &params)?
            .get())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The (unique) ID of the VM."]
    #[doc = ""]
    pub vmid: Option<crate::types::VmId>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
