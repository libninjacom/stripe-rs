use serde::{Serialize, Deserialize};
use super::PaymentMethodDomainResourcePaymentMethodStatusDetails;
///Indicates the status of a specific payment method on a payment method domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodDomainResourcePaymentMethodStatus {
    ///The status of the payment method on the domain.
    pub status: String,
    ///Contains additional details about the status of a payment method for a specific payment method domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<PaymentMethodDomainResourcePaymentMethodStatusDetails>,
}
impl std::fmt::Display for PaymentMethodDomainResourcePaymentMethodStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}