
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferSchedule {
    pub delay_days: i64,
    pub interval: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<String>,
}
impl std::fmt::Display for TransferSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}