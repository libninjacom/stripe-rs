
use serde::{Serialize, Deserialize};
use super::ShippingRate;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShippingResourcesShippingRateList {
    pub data: Vec<ShippingRate>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for ShippingResourcesShippingRateList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}