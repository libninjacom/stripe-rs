
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportingReportType {
    pub data_available_end: i64,
    pub data_available_start: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_columns: Option<Vec<String>>,
    pub id: String,
    pub livemode: bool,
    pub name: String,
    pub object: String,
    pub updated: i64,
    pub version: i64,
}
impl std::fmt::Display for ReportingReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}