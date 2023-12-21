use serde::{Serialize, Deserialize};
use super::TreasuryFinancialAccountsResourceAbaRecord;
///FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceFinancialAddress {
    ///ABA Records contain U.S. bank account details per the ABA format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<TreasuryFinancialAccountsResourceAbaRecord>,
    ///The list of networks that the address supports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<String>>,
    ///The type of financial address
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}