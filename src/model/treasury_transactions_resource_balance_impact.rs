
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryTransactionsResourceBalanceImpact {
    pub cash: i64,
    pub inbound_pending: i64,
    pub outbound_pending: i64,
}
impl std::fmt::Display for TreasuryTransactionsResourceBalanceImpact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}