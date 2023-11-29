
use serde::{Serialize, Deserialize};
use super::TreasuryCreditReversal;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedCreditsResourceCreditReversalList {
    pub data: Vec<TreasuryCreditReversal>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceCreditReversalList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}