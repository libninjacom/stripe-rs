
use serde::{Serialize, Deserialize};
use super::{PortalBusinessProfile, PortalFeatures, PortalLoginPage};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPortalConfiguration {
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    pub business_profile: PortalBusinessProfile,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<String>,
    pub features: PortalFeatures,
    pub id: String,
    pub is_default: bool,
    pub livemode: bool,
    pub login_page: PortalLoginPage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    pub updated: i64,
}
impl std::fmt::Display for BillingPortalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}