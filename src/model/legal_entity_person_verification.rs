
use serde::{Serialize, Deserialize};
use super::LegalEntityPersonVerificationDocument;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegalEntityPersonVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<LegalEntityPersonVerificationDocument>,
    pub status: String,
}
impl std::fmt::Display for LegalEntityPersonVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}