
use serde::{Serialize, Deserialize};
use super::TransferReversalList;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub amount: i64,
    pub amount_reversed: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment: Option<serde_json::Value>,
    pub id: String,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub object: String,
    pub reversals: TransferReversalList,
    pub reversed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}
impl std::fmt::Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}