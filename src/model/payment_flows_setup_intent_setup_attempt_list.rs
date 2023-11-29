
use serde::{Serialize, Deserialize};
use super::SetupAttempt;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentFlowsSetupIntentSetupAttemptList {
    pub data: Vec<SetupAttempt>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PaymentFlowsSetupIntentSetupAttemptList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}