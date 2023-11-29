
use serde::{Serialize, Deserialize};
use super::TaxRate;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionScheduleConfigurationItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub price: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
}
impl std::fmt::Display for SubscriptionScheduleConfigurationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}