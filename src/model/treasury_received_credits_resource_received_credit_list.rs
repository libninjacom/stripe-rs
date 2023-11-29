
use serde::{Serialize, Deserialize};
use super::TreasuryReceivedCredit;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedCreditsResourceReceivedCreditList {
    pub data: Vec<TreasuryReceivedCredit>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceReceivedCreditList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}