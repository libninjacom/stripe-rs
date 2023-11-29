
use serde::{Serialize, Deserialize};
use super::TaxProductResourceJurisdiction;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceLineItemTaxBreakdown {
    pub amount: i64,
    pub jurisdiction: TaxProductResourceJurisdiction,
    pub sourcing: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate_details: Option<serde_json::Value>,
    pub taxability_reason: String,
    pub taxable_amount: i64,
}
impl std::fmt::Display for TaxProductResourceLineItemTaxBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}