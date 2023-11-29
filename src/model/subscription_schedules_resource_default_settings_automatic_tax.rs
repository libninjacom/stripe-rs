
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    pub enabled: bool,
}
impl std::fmt::Display for SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}