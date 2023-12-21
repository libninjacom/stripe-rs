use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouponAppliesTo {
    ///A list of product IDs this coupon applies to
    pub products: Vec<String>,
}
impl std::fmt::Display for CouponAppliesTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}