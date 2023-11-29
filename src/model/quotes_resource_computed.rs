
use serde::{Serialize, Deserialize};
use super::QuotesResourceUpfront;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesResourceComputed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<serde_json::Value>,
    pub upfront: QuotesResourceUpfront,
}
impl std::fmt::Display for QuotesResourceComputed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}