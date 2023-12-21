use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer {
    ///The last 4 digits of the account number of the sender of the funding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number_last4: Option<String>,
    ///The full name of the sender, as supplied by the sending bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    ///The sort code of the bank of the sender of the funding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}
impl std::fmt::Display
for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}