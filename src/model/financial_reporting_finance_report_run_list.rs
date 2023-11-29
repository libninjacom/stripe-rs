
use serde::{Serialize, Deserialize};
use super::ReportingReportRun;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialReportingFinanceReportRunList {
    pub data: Vec<ReportingReportRun>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for FinancialReportingFinanceReportRunList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}