
use serde::{Serialize, Deserialize};
use super::{Price, TaxRate};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<serde_json::Value>,
    pub created: i64,
    pub id: String,
    pub metadata: serde_json::Value,
    pub object: String,
    pub price: Price,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    pub subscription: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
}
impl std::fmt::Display for SubscriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}