
use serde::{Serialize, Deserialize};
use super::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
    pub bank_transfer: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer,
}
impl std::fmt::Display
for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}