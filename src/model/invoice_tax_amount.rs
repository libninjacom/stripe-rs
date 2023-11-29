
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceTaxAmount {
    pub amount: i64,
    pub inclusive: bool,
    pub tax_rate: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxability_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<i64>,
}
impl std::fmt::Display for InvoiceTaxAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}