
use serde::{Serialize, Deserialize};
use super::PaymentMethodDomainResourcePaymentMethodStatusDetails;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodDomainResourcePaymentMethodStatus {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<PaymentMethodDomainResourcePaymentMethodStatusDetails>,
}
impl std::fmt::Display for PaymentMethodDomainResourcePaymentMethodStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}