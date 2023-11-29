
use serde::{Serialize, Deserialize};
use super::TreasuryTransactionsResourceBalanceImpact;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryTransactionEntry {
    pub balance_impact: TreasuryTransactionsResourceBalanceImpact,
    pub created: i64,
    pub currency: String,
    pub effective_at: i64,
    pub financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_details: Option<serde_json::Value>,
    pub flow_type: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub transaction: serde_json::Value,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TreasuryTransactionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}