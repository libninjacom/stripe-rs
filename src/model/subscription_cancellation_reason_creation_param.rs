
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCancellationReasonCreationParam {
    pub enabled: bool,
    pub options: serde_json::Value,
}
impl std::fmt::Display for SubscriptionCancellationReasonCreationParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}