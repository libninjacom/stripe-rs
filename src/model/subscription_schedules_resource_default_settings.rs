
use serde::{Serialize, Deserialize};
use super::SubscriptionSchedulesResourceDefaultSettingsAutomaticTax;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionSchedulesResourceDefaultSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<SubscriptionSchedulesResourceDefaultSettingsAutomaticTax>,
    pub billing_cycle_anchor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<serde_json::Value>,
}
impl std::fmt::Display for SubscriptionSchedulesResourceDefaultSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}