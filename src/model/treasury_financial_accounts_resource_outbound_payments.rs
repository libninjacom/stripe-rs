use serde::{Serialize, Deserialize};
use super::{
    TreasuryFinancialAccountsResourceAchToggleSettings,
    TreasuryFinancialAccountsResourceToggleSettings,
};
///Settings related to Outbound Payments features on a Financial Account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceOutboundPayments {
    ///Toggle settings for enabling/disabling an ACH specific feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<TreasuryFinancialAccountsResourceAchToggleSettings>,
    ///Toggle settings for enabling/disabling a feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceOutboundPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}