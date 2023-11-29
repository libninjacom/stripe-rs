
use serde::{Serialize, Deserialize};
use super::PortalSubscriptionUpdateProduct;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalSubscriptionUpdate {
    pub default_allowed_updates: Vec<String>,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<PortalSubscriptionUpdateProduct>>,
    pub proration_behavior: String,
}
impl std::fmt::Display for PortalSubscriptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}