
use serde::{Serialize, Deserialize};
use super::CouponOfferParam;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionParam {
    pub coupon_offer: CouponOfferParam,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for RetentionParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}