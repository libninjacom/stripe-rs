use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentsFeaturesParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_payments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}
impl std::fmt::Display for PaymentsFeaturesParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}