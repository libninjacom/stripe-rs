
use serde::{Serialize, Deserialize};
use super::PaymentMethodDomain;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodDomainResourcePaymentMethodDomainList {
    pub data: Vec<PaymentMethodDomain>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PaymentMethodDomainResourcePaymentMethodDomainList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}