
use serde::{Serialize, Deserialize};
use super::{BalanceTransaction, IssuingDisputeEvidence};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingDispute {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transactions: Option<Vec<BalanceTransaction>>,
    pub created: i64,
    pub currency: String,
    pub evidence: IssuingDisputeEvidence,
    pub id: String,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub object: String,
    pub status: String,
    pub transaction: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingDispute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}