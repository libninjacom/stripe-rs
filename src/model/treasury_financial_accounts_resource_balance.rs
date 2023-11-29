
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceBalance {
    pub cash: serde_json::Value,
    pub inbound_pending: serde_json::Value,
    pub outbound_pending: serde_json::Value,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}