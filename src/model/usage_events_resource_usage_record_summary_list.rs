
use serde::{Serialize, Deserialize};
use super::UsageRecordSummary;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageEventsResourceUsageRecordSummaryList {
    pub data: Vec<UsageRecordSummary>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for UsageEventsResourceUsageRecordSummaryList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}