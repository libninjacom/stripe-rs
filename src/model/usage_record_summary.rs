
use serde::{Serialize, Deserialize};
use super::Period;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecordSummary {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    pub livemode: bool,
    pub object: String,
    pub period: Period,
    pub subscription_item: String,
    pub total_usage: i64,
}
impl std::fmt::Display for UsageRecordSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}