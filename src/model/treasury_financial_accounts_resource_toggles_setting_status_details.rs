use serde::{Serialize, Deserialize};
///Additional details on the FinancialAccount Features information.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
    ///Represents the reason why the status is `pending` or `restricted`.
    pub code: String,
    ///Represents what the user should do, if anything, to activate the Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    ///The `platform_restrictions` that are restricting this Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction: Option<String>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}