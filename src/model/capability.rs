
use serde::{Serialize, Deserialize};
use super::{AccountCapabilityFutureRequirements, AccountCapabilityRequirements};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub account: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<AccountCapabilityFutureRequirements>,
    pub id: String,
    pub object: String,
    pub requested: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<AccountCapabilityRequirements>,
    pub status: String,
}
impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}