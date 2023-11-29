
use serde::{Serialize, Deserialize};
use super::PriceTier;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub active: bool,
    pub billing_scheme: String,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<serde_json::Value>,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,
    pub metadata: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub object: String,
    pub product: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PriceTier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub unit_amount_decimal: Option<rust_decimal::Decimal>,
}
impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}