use serde::{Serialize, Deserialize};
use super::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails;
///Toggle settings for enabling/disabling the ABA address feature
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceAbaToggleSettings {
    ///Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    ///Whether the Feature is operational.
    pub status: String,
    ///Additional details; includes at least one entry when the status is not `active`.
    pub status_details: Vec<
        TreasuryFinancialAccountsResourceTogglesSettingStatusDetails,
    >,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceAbaToggleSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}