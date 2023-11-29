
use serde::{Serialize, Deserialize};
use super::CustomerBalanceTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerBalanceTransactionList {
    pub data: Vec<CustomerBalanceTransaction>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for CustomerBalanceTransactionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}