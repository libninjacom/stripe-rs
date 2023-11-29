
use serde::{Serialize, Deserialize};
use super::ClimateSupplier;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateProduct {
    pub created: i64,
    pub current_prices_per_metric_ton: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_year: Option<i64>,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub metric_tons_available: Option<rust_decimal::Decimal>,
    pub name: String,
    pub object: String,
    pub suppliers: Vec<ClimateSupplier>,
}
impl std::fmt::Display for ClimateProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}