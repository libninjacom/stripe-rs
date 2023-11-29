
use serde::{Serialize, Deserialize};
use super::TaxProductResourceTaxRateDetails;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxBreakdown {
    pub amount: i64,
    pub inclusive: bool,
    pub tax_rate_details: TaxProductResourceTaxRateDetails,
    pub taxability_reason: String,
    pub taxable_amount: i64,
}
impl std::fmt::Display for TaxProductResourceTaxBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}