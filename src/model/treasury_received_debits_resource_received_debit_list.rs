
use serde::{Serialize, Deserialize};
use super::TreasuryReceivedDebit;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedDebitsResourceReceivedDebitList {
    pub data: Vec<TreasuryReceivedDebit>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceReceivedDebitList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}