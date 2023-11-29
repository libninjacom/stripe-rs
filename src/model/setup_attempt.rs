
use serde::{Serialize, Deserialize};
use super::SetupAttemptPaymentMethodDetails;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupAttempt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_directions: Option<Vec<String>>,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    pub payment_method: serde_json::Value,
    pub payment_method_details: SetupAttemptPaymentMethodDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_error: Option<serde_json::Value>,
    pub setup_intent: serde_json::Value,
    pub status: String,
    pub usage: String,
}
impl std::fmt::Display for SetupAttempt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}