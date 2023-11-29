
use serde::{Serialize, Deserialize};
use super::{AccountRequirementsAlternative, AccountRequirementsError};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountFutureRequirements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currently_due: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AccountRequirementsError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventually_due: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_verification: Option<Vec<String>>,
}
impl std::fmt::Display for AccountFutureRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}