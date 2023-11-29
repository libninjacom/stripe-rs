
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Payout {
    pub amount: i64,
    pub arrival_date: i64,
    pub automatic: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub method: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_payout: Option<serde_json::Value>,
    pub reconciliation_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversed_by: Option<serde_json::Value>,
    pub source_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for Payout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}