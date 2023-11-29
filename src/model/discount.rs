
use serde::{Serialize, Deserialize};
use super::Coupon;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_session: Option<String>,
    pub coupon: Coupon,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<serde_json::Value>,
    pub start: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
}
impl std::fmt::Display for Discount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}