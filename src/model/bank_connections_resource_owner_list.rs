
use serde::{Serialize, Deserialize};
use super::FinancialConnectionsAccountOwner;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankConnectionsResourceOwnerList {
    pub data: Vec<FinancialConnectionsAccountOwner>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for BankConnectionsResourceOwnerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}