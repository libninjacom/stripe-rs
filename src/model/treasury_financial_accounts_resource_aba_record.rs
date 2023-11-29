
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceAbaRecord {
    pub account_holder_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    pub account_number_last4: String,
    pub bank_name: String,
    pub routing_number: String,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceAbaRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}