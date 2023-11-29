
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentIntentProcessingCustomerNotification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_requested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completes_at: Option<i64>,
}
impl std::fmt::Display for PaymentIntentProcessingCustomerNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}