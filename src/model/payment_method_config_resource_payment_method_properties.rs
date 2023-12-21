use serde::{Serialize, Deserialize};
use super::PaymentMethodConfigResourceDisplayPreference;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodConfigResourcePaymentMethodProperties {
    ///Whether this payment method may be offered at checkout. True if `display_preference` is `on` and the payment method's capability is active.
    pub available: bool,
    ///
    pub display_preference: PaymentMethodConfigResourceDisplayPreference,
}
impl std::fmt::Display for PaymentMethodConfigResourcePaymentMethodProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}