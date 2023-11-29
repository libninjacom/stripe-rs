
use serde::{Serialize, Deserialize};
use super::{AccountRequirementsAlternative, AccountRequirementsError};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountCapabilityRequirements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<i64>,
    pub currently_due: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    pub errors: Vec<AccountRequirementsError>,
    pub eventually_due: Vec<String>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}
impl std::fmt::Display for AccountCapabilityRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}