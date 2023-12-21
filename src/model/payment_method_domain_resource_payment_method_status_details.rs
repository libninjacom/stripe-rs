use serde::{Serialize, Deserialize};
///Contains additional details about the status of a payment method for a specific payment method domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodDomainResourcePaymentMethodStatusDetails {
    ///The error message associated with the status of the payment method on the domain.
    pub error_message: String,
}
impl std::fmt::Display for PaymentMethodDomainResourcePaymentMethodStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}