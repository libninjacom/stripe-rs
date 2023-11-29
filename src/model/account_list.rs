
use serde::{Serialize, Deserialize};
use super::Account;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountList {
    pub data: Vec<Account>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for AccountList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}