
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxTransactionLineItem {
    pub amount: i64,
    pub amount_tax: i64,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    pub quantity: i64,
    pub reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal: Option<serde_json::Value>,
    pub tax_behavior: String,
    pub tax_code: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TaxTransactionLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}