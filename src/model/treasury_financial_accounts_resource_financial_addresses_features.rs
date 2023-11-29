
use serde::{Serialize, Deserialize};
use super::TreasuryFinancialAccountsResourceAbaToggleSettings;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<TreasuryFinancialAccountsResourceAbaToggleSettings>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}