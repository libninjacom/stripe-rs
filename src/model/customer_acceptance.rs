use serde::{Serialize, Deserialize};
use super::{OfflineAcceptance, OnlineAcceptance};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerAcceptance {
    ///The time that the customer accepts the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<i64>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<OfflineAcceptance>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<OnlineAcceptance>,
    ///The mandate includes the type of customer acceptance information, such as: `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for CustomerAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}