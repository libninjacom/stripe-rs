
use serde::{Serialize, Deserialize};
use super::{SubscriptionAutomaticTax, SubscriptionItemList, TaxRate};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: SubscriptionAutomaticTax,
    pub billing_cycle_anchor: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<i64>,
    pub cancel_at_period_end: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_details: Option<serde_json::Value>,
    pub collection_method: String,
    pub created: i64,
    pub currency: String,
    pub current_period_end: i64,
    pub current_period_start: i64,
    pub customer: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<TaxRate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<i64>,
    pub id: String,
    pub items: SubscriptionItemList,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_invoice: Option<serde_json::Value>,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_pending_invoice_item_invoice: Option<i64>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_collection: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_setup_intent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_update: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<serde_json::Value>,
    pub start_date: i64,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_start: Option<i64>,
}
impl std::fmt::Display for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}