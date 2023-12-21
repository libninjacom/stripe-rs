use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceStatusDetails {
    ///Details related to the closure of this FinancialAccount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}