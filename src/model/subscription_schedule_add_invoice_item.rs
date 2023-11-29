
use serde::{Serialize, Deserialize};
use super::TaxRate;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionScheduleAddInvoiceItem {
    pub price: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
}
impl std::fmt::Display for SubscriptionScheduleAddInvoiceItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}