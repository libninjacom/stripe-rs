
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<i64>,
}
impl std::fmt::Display for IssuingDisputeMerchandiseNotAsDescribedEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}