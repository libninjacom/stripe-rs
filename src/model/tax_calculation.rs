
use serde::{Serialize, Deserialize};
use super::{
    TaxProductResourceCustomerDetails, TaxProductResourceTaxBreakdown,
    TaxProductResourceTaxCalculationLineItemList,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCalculation {
    pub amount_total: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    pub customer_details: TaxProductResourceCustomerDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<TaxProductResourceTaxCalculationLineItemList>,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<serde_json::Value>,
    pub tax_amount_exclusive: i64,
    pub tax_amount_inclusive: i64,
    pub tax_breakdown: Vec<TaxProductResourceTaxBreakdown>,
    pub tax_date: i64,
}
impl std::fmt::Display for TaxCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}