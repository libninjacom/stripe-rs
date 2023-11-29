
use serde::{Serialize, Deserialize};
use super::TaxRate;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItemsTaxAmount {
    pub amount: i64,
    pub rate: TaxRate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxability_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<i64>,
}
impl std::fmt::Display for LineItemsTaxAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}