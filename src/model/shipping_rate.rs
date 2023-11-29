
use serde::{Serialize, Deserialize};
use super::ShippingRateFixedAmount;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingRate {
    pub active: bool,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<ShippingRateFixedAmount>,
    pub id: String,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for ShippingRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}