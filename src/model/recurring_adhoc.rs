
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringAdhoc {
    pub interval: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i64>,
}
impl std::fmt::Display for RecurringAdhoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}