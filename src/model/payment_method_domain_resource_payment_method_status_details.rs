
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodDomainResourcePaymentMethodStatusDetails {
    pub error_message: String,
}
impl std::fmt::Display for PaymentMethodDomainResourcePaymentMethodStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}