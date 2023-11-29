
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalAccountList {
    pub data: Vec<serde_json::Value>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for ExternalAccountList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}