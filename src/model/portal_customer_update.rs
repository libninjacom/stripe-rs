
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalCustomerUpdate {
    pub allowed_updates: Vec<String>,
    pub enabled: bool,
}
impl std::fmt::Display for PortalCustomerUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}