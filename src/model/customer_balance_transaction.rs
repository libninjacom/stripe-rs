
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceTransaction {
    pub amount: i64,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note: Option<serde_json::Value>,
    pub currency: String,
    pub customer: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub ending_balance: i64,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for CustomerBalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}