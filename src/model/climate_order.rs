
use serde::{Serialize, Deserialize};
use super::{ClimateRemovalsBeneficiary, ClimateRemovalsOrderDeliveries};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateOrder {
    pub amount_fees: i64,
    pub amount_subtotal: i64,
    pub amount_total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<ClimateRemovalsBeneficiary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmed_at: Option<i64>,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delayed_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<i64>,
    pub delivery_details: Vec<ClimateRemovalsOrderDeliveries>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_delivery_year: Option<i64>,
    pub id: String,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    #[serde(with = "rust_decimal::serde::str")]
    pub metric_tons: rust_decimal::Decimal,
    pub object: String,
    pub product: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_substituted_at: Option<i64>,
    pub status: String,
}
impl std::fmt::Display for ClimateOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}