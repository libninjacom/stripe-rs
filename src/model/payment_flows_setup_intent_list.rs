
use serde::{Serialize, Deserialize};
use super::SetupIntent;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentFlowsSetupIntentList {
    pub data: Vec<SetupIntent>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PaymentFlowsSetupIntentList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}