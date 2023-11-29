
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}