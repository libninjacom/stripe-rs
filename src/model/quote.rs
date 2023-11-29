
use serde::{Serialize, Deserialize};
use super::{
    QuotesResourceAutomaticTax, QuotesResourceComputed, QuotesResourceListLineItems,
    QuotesResourceStatusTransitions, QuotesResourceSubscriptionDataSubscriptionData,
    QuotesResourceTotalDetails,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub amount_subtotal: i64,
    pub amount_total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: QuotesResourceAutomaticTax,
    pub collection_method: String,
    pub computed: QuotesResourceComputed,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub discounts: Vec<serde_json::Value>,
    pub expires_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_quote: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<QuotesResourceListLineItems>,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    pub status: String,
    pub status_transitions: QuotesResourceStatusTransitions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<serde_json::Value>,
    pub subscription_data: QuotesResourceSubscriptionDataSubscriptionData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_schedule: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<serde_json::Value>,
    pub total_details: QuotesResourceTotalDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<serde_json::Value>,
}
impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}