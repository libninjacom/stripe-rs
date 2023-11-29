
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLink {
    pub created: i64,
    pub expired: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    pub file: serde_json::Value,
    pub id: String,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for FileLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}