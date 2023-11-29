
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEventData {
    pub object: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<serde_json::Value>,
}
impl std::fmt::Display for NotificationEventData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}