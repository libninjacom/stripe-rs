
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundNextAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_details: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for RefundNextAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}