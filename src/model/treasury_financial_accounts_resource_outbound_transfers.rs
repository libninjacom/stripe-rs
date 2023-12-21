use serde::{Serialize, Deserialize};
use super::{
    TreasuryFinancialAccountsResourceAchToggleSettings,
    TreasuryFinancialAccountsResourceToggleSettings,
};
///OutboundTransfers contains outbound transfers features for a FinancialAccount.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {
    ///Toggle settings for enabling/disabling an ACH specific feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<TreasuryFinancialAccountsResourceAchToggleSettings>,
    ///Toggle settings for enabling/disabling a feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceOutboundTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}