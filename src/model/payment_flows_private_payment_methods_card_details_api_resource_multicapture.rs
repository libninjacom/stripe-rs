
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
    pub status: String,
}
impl std::fmt::Display
for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}