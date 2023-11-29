
use serde::{Serialize, Deserialize};
use super::{
    TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
    TreasuryTransactionsResourceBalanceImpact,
    TreasuryTransactionsResourceTransactionEntryList,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryTransaction {
    pub amount: i64,
    pub balance_impact: TreasuryTransactionsResourceBalanceImpact,
    pub created: i64,
    pub currency: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<TreasuryTransactionsResourceTransactionEntryList>,
    pub financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_details: Option<serde_json::Value>,
    pub flow_type: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub status: String,
    pub status_transitions: TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
}
impl std::fmt::Display for TreasuryTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}