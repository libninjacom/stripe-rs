
use serde::{Serialize, Deserialize};
use super::TreasuryDebitReversal;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedDebitsResourceDebitReversalList {
    pub data: Vec<TreasuryDebitReversal>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceDebitReversalList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}