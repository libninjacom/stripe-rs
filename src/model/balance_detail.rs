
use serde::{Serialize, Deserialize};
use super::BalanceAmount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BalanceDetail {
    pub available: Vec<BalanceAmount>,
}
impl std::fmt::Display for BalanceDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}