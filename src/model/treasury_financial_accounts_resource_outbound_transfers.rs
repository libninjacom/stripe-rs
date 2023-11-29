
use serde::{Serialize, Deserialize};
use super::{
    TreasuryFinancialAccountsResourceAchToggleSettings,
    TreasuryFinancialAccountsResourceToggleSettings,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<TreasuryFinancialAccountsResourceAchToggleSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceOutboundTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}