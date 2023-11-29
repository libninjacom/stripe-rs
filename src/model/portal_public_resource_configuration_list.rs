
use serde::{Serialize, Deserialize};
use super::BillingPortalConfiguration;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalPublicResourceConfigurationList {
    pub data: Vec<BillingPortalConfiguration>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PortalPublicResourceConfigurationList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}