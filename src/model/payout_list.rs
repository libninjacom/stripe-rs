
use serde::{Serialize, Deserialize};
use super::Payout;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayoutList {
    pub data: Vec<Payout>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PayoutList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}