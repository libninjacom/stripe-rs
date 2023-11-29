
use serde::{Serialize, Deserialize};
use super::IssuingCardSpendingLimit;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingCardAuthorizationControls {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<IssuingCardSpendingLimit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits_currency: Option<String>,
}
impl std::fmt::Display for IssuingCardAuthorizationControls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}