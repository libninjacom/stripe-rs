
use serde::{Serialize, Deserialize};
use super::PaymentMethodConfiguration;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodConfigResourcePaymentMethodConfigurationsList {
    pub data: Vec<PaymentMethodConfiguration>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PaymentMethodConfigResourcePaymentMethodConfigurationsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}