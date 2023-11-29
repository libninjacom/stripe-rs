
use serde::{Serialize, Deserialize};
use super::Discount;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItemsDiscountAmount {
    pub amount: i64,
    pub discount: Discount,
}
impl std::fmt::Display for LineItemsDiscountAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}