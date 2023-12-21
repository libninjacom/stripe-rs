use serde::{Serialize, Deserialize};
use super::PaymentFlowsAmountDetailsResourceTip;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentFlowsAmountDetails {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<PaymentFlowsAmountDetailsResourceTip>,
}
impl std::fmt::Display for PaymentFlowsAmountDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}