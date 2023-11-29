
use serde::{Serialize, Deserialize};
use super::TreasuryFinancialAccount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceFinancialAccountList {
    pub data: Vec<TreasuryFinancialAccount>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAccountList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}