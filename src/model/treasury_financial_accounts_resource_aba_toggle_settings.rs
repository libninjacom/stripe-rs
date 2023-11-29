
use serde::{Serialize, Deserialize};
use super::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceAbaToggleSettings {
    pub requested: bool,
    pub status: String,
    pub status_details: Vec<
        TreasuryFinancialAccountsResourceTogglesSettingStatusDetails,
    >,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceAbaToggleSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}