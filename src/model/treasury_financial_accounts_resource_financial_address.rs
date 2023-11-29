
use serde::{Serialize, Deserialize};
use super::TreasuryFinancialAccountsResourceAbaRecord;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceFinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<TreasuryFinancialAccountsResourceAbaRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}