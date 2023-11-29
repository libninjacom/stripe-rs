
use serde::{Serialize, Deserialize};
use super::Charge;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchResult {
    pub data: Vec<Charge>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    pub url: String,
}
impl std::fmt::Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}