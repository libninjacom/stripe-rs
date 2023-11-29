
use serde::{Serialize, Deserialize};
use super::Fee;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BalanceTransaction {
    pub amount: i64,
    pub available_on: i64,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<f64>,
    pub fee: i64,
    pub fee_details: Vec<Fee>,
    pub id: String,
    pub net: i64,
    pub object: String,
    pub reporting_category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for BalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}