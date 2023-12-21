use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer {
    ///The name of the bank of the sender of the funding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_bank: Option<String>,
    ///The name of the bank branch of the sender of the funding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_branch: Option<String>,
    ///The full name of the sender, as supplied by the sending bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
}
impl std::fmt::Display
for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}