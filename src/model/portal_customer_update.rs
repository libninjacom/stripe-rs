use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalCustomerUpdate {
    ///The types of customer updates that are supported. When empty, customers are not updateable.
    pub allowed_updates: Vec<String>,
    ///Whether the feature is enabled.
    pub enabled: bool,
}
impl std::fmt::Display for PortalCustomerUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}