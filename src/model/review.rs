
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Review {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_reason: Option<String>,
    pub created: i64,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_location: Option<serde_json::Value>,
    pub livemode: bool,
    pub object: String,
    pub open: bool,
    pub opened_reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<serde_json::Value>,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<serde_json::Value>,
}
impl std::fmt::Display for Review {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}