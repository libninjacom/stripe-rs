
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalSubscriptionUpdateProduct {
    pub prices: Vec<String>,
    pub product: String,
}
impl std::fmt::Display for PortalSubscriptionUpdateProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}