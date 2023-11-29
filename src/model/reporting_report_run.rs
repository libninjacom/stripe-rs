
use serde::{Serialize, Deserialize};
use super::FinancialReportingFinanceReportRunRunParameters;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingReportRun {
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub parameters: FinancialReportingFinanceReportRunRunParameters,
    pub report_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<i64>,
}
impl std::fmt::Display for ReportingReportRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}