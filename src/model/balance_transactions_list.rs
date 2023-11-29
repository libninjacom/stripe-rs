
use serde::{Serialize, Deserialize};
use super::BalanceTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BalanceTransactionsList {
    pub data: Vec<BalanceTransaction>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for BalanceTransactionsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}