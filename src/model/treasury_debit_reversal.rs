
use serde::{Serialize, Deserialize};
use super::TreasuryReceivedDebitsResourceStatusTransitions;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryDebitReversal {
    pub amount: i64,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_flows: Option<serde_json::Value>,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub network: String,
    pub object: String,
    pub received_debit: String,
    pub status: String,
    pub status_transitions: TreasuryReceivedDebitsResourceStatusTransitions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryDebitReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}