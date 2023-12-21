use serde::{Serialize, Deserialize};
use super::TreasuryFinancialAccountsResourceAbaToggleSettings;
///Settings related to Financial Addresses features on a Financial Account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    ///Toggle settings for enabling/disabling the ABA address feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<TreasuryFinancialAccountsResourceAbaToggleSettings>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}