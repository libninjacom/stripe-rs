
use serde::{Serialize, Deserialize};
use super::SigmaScheduledQueryRunError;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScheduledQueryRun {
    pub created: i64,
    pub data_load_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<SigmaScheduledQueryRunError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<serde_json::Value>,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub result_available_until: i64,
    pub sql: String,
    pub status: String,
    pub title: String,
}
impl std::fmt::Display for ScheduledQueryRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}