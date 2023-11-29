
use serde::{Serialize, Deserialize};
use super::{
    SchedulesPhaseAutomaticTax, SubscriptionScheduleAddInvoiceItem,
    SubscriptionScheduleConfigurationItem, TaxRate,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionSchedulePhaseConfiguration {
    pub add_invoice_items: Vec<SubscriptionScheduleAddInvoiceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<SchedulesPhaseAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<serde_json::Value>,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<TaxRate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub end_date: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<serde_json::Value>,
    pub items: Vec<SubscriptionScheduleConfigurationItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    pub proration_behavior: String,
    pub start_date: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,
}
impl std::fmt::Display for SubscriptionSchedulePhaseConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}