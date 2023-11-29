
use serde::{Serialize, Deserialize};
use super::PaymentMethodDomainResourcePaymentMethodStatus;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDomain {
    pub apple_pay: PaymentMethodDomainResourcePaymentMethodStatus,
    pub created: i64,
    pub domain_name: String,
    pub enabled: bool,
    pub google_pay: PaymentMethodDomainResourcePaymentMethodStatus,
    pub id: String,
    pub link: PaymentMethodDomainResourcePaymentMethodStatus,
    pub livemode: bool,
    pub object: String,
    pub paypal: PaymentMethodDomainResourcePaymentMethodStatus,
}
impl std::fmt::Display for PaymentMethodDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}