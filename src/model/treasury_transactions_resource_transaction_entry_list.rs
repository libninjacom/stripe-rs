
use serde::{Serialize, Deserialize};
use super::TreasuryTransactionEntry;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryTransactionsResourceTransactionEntryList {
    pub data: Vec<TreasuryTransactionEntry>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TreasuryTransactionsResourceTransactionEntryList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}