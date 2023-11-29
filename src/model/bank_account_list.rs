
use serde::{Serialize, Deserialize};
use super::BankAccount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankAccountList {
    pub data: Vec<BankAccount>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for BankAccountList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}