
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
    pub balance_transaction: serde_json::Value,
}
impl std::fmt::Display
for CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}