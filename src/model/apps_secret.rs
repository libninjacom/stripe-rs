
use serde::{Serialize, Deserialize};
use super::SecretServiceResourceScope;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppsSecret {
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    pub id: String,
    pub livemode: bool,
    pub name: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    pub scope: SecretServiceResourceScope,
}
impl std::fmt::Display for AppsSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}