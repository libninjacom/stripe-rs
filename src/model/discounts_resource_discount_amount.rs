use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscountsResourceDiscountAmount {
    ///The amount, in cents (or local equivalent), of the discount.
    pub amount: i64,
    ///The discount that was applied to get this discount amount.
    pub discount: serde_json::Value,
}
impl std::fmt::Display for DiscountsResourceDiscountAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}