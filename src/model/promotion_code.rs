
use serde::{Serialize, Deserialize};
use super::{Coupon, PromotionCodesResourceRestrictions};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionCode {
    pub active: bool,
    pub code: String,
    pub coupon: Coupon,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    pub restrictions: PromotionCodesResourceRestrictions,
    pub times_redeemed: i64,
}
impl std::fmt::Display for PromotionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}