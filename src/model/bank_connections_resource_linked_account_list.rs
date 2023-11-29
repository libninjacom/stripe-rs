
use serde::{Serialize, Deserialize};
use super::FinancialConnectionsAccount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankConnectionsResourceLinkedAccountList {
    pub data: Vec<FinancialConnectionsAccount>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for BankConnectionsResourceLinkedAccountList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}