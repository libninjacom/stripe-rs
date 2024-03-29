use serde::{Serialize, Deserialize};
use super::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails;
///Toggle settings for enabling/disabling a feature
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceToggleSettings {
    ///Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    ///Whether the Feature is operational.
    pub status: String,
    ///Additional details; includes at least one entry when the status is not `active`.
    pub status_details: Vec<
        TreasuryFinancialAccountsResourceTogglesSettingStatusDetails,
    >,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceToggleSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}