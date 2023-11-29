
use serde::{Serialize, Deserialize};
use super::TreasuryTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryTransactionsResourceTransactionList {
    pub data: Vec<TreasuryTransaction>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TreasuryTransactionsResourceTransactionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}