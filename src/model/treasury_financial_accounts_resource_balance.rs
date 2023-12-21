use serde::{Serialize, Deserialize};
///Balance information for the FinancialAccount
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountsResourceBalance {
    ///Funds the user can spend right now.
    pub cash: serde_json::Value,
    ///Funds not spendable yet, but will become available at a later time.
    pub inbound_pending: serde_json::Value,
    ///Funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: serde_json::Value,
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}