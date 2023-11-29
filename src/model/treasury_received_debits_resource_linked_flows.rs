
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedDebitsResourceLinkedFlows {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_reversal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_transaction: Option<String>,
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}