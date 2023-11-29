
use serde::{Serialize, Deserialize};
use super::Refund;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundList {
    pub data: Vec<Refund>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for RefundList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}