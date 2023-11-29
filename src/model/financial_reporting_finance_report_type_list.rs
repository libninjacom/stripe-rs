
use serde::{Serialize, Deserialize};
use super::ReportingReportType;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialReportingFinanceReportTypeList {
    pub data: Vec<ReportingReportType>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for FinancialReportingFinanceReportTypeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}