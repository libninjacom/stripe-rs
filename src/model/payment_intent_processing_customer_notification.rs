use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentIntentProcessingCustomerNotification {
    ///Whether customer approval has been requested for this payment. For payments greater than INR 15000 or mandate amount, the customer must provide explicit approval of the payment with their bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_requested: Option<bool>,
    ///If customer approval is required, they need to provide approval before this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completes_at: Option<i64>,
}
impl std::fmt::Display for PaymentIntentProcessingCustomerNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}