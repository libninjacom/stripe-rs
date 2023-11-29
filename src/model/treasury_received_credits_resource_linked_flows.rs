
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedCreditsResourceLinkedFlows {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_transaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow_type: Option<String>,
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}