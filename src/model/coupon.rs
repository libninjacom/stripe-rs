
use serde::{Serialize, Deserialize};
use super::CouponAppliesTo;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Coupon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<CouponAppliesTo>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<serde_json::Value>,
    pub duration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_months: Option<i64>,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_by: Option<i64>,
    pub times_redeemed: i64,
    pub valid: bool,
}
impl std::fmt::Display for Coupon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}