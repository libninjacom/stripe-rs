
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialReportingFinanceReportRunRunParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}
impl std::fmt::Display for FinancialReportingFinanceReportRunRunParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}