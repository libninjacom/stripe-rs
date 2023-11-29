
use serde::{Serialize, Deserialize};
use super::ClimateSupplier;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateRemovalsOrderDeliveries {
    pub delivered_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,
    pub metric_tons: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
    pub supplier: ClimateSupplier,
}
impl std::fmt::Display for ClimateRemovalsOrderDeliveries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}