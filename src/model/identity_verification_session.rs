
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub created: i64,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_verification_report: Option<serde_json::Value>,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction: Option<serde_json::Value>,
    pub status: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_outputs: Option<serde_json::Value>,
}
impl std::fmt::Display for IdentityVerificationSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}