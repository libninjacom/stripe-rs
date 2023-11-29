
use serde::{Serialize, Deserialize};
use super::TreasuryFinancialAccountsResourceAchToggleSettings;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceInboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<TreasuryFinancialAccountsResourceAchToggleSettings>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceInboundTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}