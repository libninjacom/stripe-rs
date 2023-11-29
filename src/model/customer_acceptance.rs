
use serde::{Serialize, Deserialize};
use super::{OfflineAcceptance, OnlineAcceptance};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerAcceptance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<OfflineAcceptance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<OnlineAcceptance>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for CustomerAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}