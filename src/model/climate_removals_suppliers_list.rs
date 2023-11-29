
use serde::{Serialize, Deserialize};
use super::ClimateSupplier;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClimateRemovalsSuppliersList {
    pub data: Vec<ClimateSupplier>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for ClimateRemovalsSuppliersList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}