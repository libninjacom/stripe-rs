
use serde::{Serialize, Deserialize};
use super::Coupon;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouponsResourceCouponList {
    pub data: Vec<Coupon>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for CouponsResourceCouponList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}