
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuotesResourceStatusTransitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<i64>,
}
impl std::fmt::Display for QuotesResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}