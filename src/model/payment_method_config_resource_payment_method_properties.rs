
use serde::{Serialize, Deserialize};
use super::PaymentMethodConfigResourceDisplayPreference;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodConfigResourcePaymentMethodProperties {
    pub available: bool,
    pub display_preference: PaymentMethodConfigResourceDisplayPreference,
}
impl std::fmt::Display for PaymentMethodConfigResourcePaymentMethodProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}