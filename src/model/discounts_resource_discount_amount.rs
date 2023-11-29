
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountsResourceDiscountAmount {
    pub amount: i64,
    pub discount: serde_json::Value,
}
impl std::fmt::Display for DiscountsResourceDiscountAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}