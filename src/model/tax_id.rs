
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<serde_json::Value>,
}
impl std::fmt::Display for TaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}