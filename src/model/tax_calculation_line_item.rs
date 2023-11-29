
use serde::{Serialize, Deserialize};
use super::TaxProductResourceLineItemTaxBreakdown;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxCalculationLineItem {
    pub amount: i64,
    pub amount_tax: i64,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    pub quantity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    pub tax_behavior: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_breakdown: Option<Vec<TaxProductResourceLineItemTaxBreakdown>>,
    pub tax_code: String,
}
impl std::fmt::Display for TaxCalculationLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}