
use serde::{Serialize, Deserialize};
use super::{IssuingCardholderAddress, IssuingCardholderRequirements};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingCardholder {
    pub billing: IssuingCardholderAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<serde_json::Value>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<serde_json::Value>,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub name: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    pub requirements: IssuingCardholderRequirements,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_controls: Option<serde_json::Value>,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for IssuingCardholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}