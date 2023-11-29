
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegalEntityPersonVerificationDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<serde_json::Value>,
}
impl std::fmt::Display for LegalEntityPersonVerificationDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}