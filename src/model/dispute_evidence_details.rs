
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeEvidenceDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_by: Option<i64>,
    pub has_evidence: bool,
    pub past_due: bool,
    pub submission_count: i64,
}
impl std::fmt::Display for DisputeEvidenceDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}