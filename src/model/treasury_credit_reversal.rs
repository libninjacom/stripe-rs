
use serde::{Serialize, Deserialize};
use super::TreasuryReceivedCreditsResourceStatusTransitions;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryCreditReversal {
    pub amount: i64,
    pub created: i64,
    pub currency: String,
    pub financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    pub id: String,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub network: String,
    pub object: String,
    pub received_credit: String,
    pub status: String,
    pub status_transitions: TreasuryReceivedCreditsResourceStatusTransitions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryCreditReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}