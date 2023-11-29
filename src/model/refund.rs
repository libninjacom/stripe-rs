
use serde::{Serialize, Deserialize};
use super::RefundNextAction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Refund {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<serde_json::Value>,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<RefundNextAction>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transfer_reversal: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_reversal: Option<serde_json::Value>,
}
impl std::fmt::Display for Refund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}