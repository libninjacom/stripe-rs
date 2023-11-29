
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentFlowsAmountDetailsResourceTip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
impl std::fmt::Display for PaymentFlowsAmountDetailsResourceTip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}