use serde::{Serialize, Deserialize};
use super::ClimateSupplier;
///The delivery of a specified quantity of carbon for an order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClimateRemovalsOrderDeliveries {
    ///Time at which the delivery occurred. Measured in seconds since the Unix epoch.
    pub delivered_at: i64,
    ///Specific location of this delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,
    ///Quantity of carbon removal supplied by this delivery.
    pub metric_tons: String,
    ///Once retired, a URL to the registry entry for the tons from this delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
    ///A supplier of carbon removal.
    pub supplier: ClimateSupplier,
}
impl std::fmt::Display for ClimateRemovalsOrderDeliveries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}