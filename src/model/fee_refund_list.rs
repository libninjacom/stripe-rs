
use serde::{Serialize, Deserialize};
use super::FeeRefund;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeeRefundList {
    pub data: Vec<FeeRefund>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for FeeRefundList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}