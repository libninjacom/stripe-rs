use serde::{Serialize, Deserialize};
use super::DeliveryEstimateBound;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveryEstimate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<DeliveryEstimateBound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<DeliveryEstimateBound>,
}
impl std::fmt::Display for DeliveryEstimate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}