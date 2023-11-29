
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoicesStatusTransitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marked_uncollectible_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<i64>,
}
impl std::fmt::Display for InvoicesStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}