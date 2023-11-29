
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPortalSession {
    pub configuration: serde_json::Value,
    pub created: i64,
    pub customer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<serde_json::Value>,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    pub url: String,
}
impl std::fmt::Display for BillingPortalSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}