
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferReversal {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_refund: Option<serde_json::Value>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_refund: Option<serde_json::Value>,
    pub transfer: serde_json::Value,
}
impl std::fmt::Display for TransferReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}