
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
    pub balance_transaction: serde_json::Value,
    pub linked_transaction: serde_json::Value,
}
impl std::fmt::Display
for CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}