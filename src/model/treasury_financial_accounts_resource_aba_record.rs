use serde::{Serialize, Deserialize};
///ABA Records contain U.S. bank account details per the ABA format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceAbaRecord {
    ///The name of the person or business that owns the bank account.
    pub account_holder_name: String,
    ///The account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    ///The last four characters of the account number.
    pub account_number_last4: String,
    ///Name of the bank.
    pub bank_name: String,
    ///Routing number for the account.
    pub routing_number: String,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceAbaRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}