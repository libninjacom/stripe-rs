
use serde::{Serialize, Deserialize};
use super::PaymentMethod;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerPaymentMethodResourceList {
    pub data: Vec<PaymentMethod>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for CustomerPaymentMethodResourceList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}