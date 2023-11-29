
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingDisputeDuplicateEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_statement: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_receipt: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transaction: Option<String>,
}
impl std::fmt::Display for IssuingDisputeDuplicateEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}