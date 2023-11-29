
use serde::{Serialize, Deserialize};
use super::ClimateOrder;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClimateRemovalsOrdersList {
    pub data: Vec<ClimateOrder>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for ClimateRemovalsOrdersList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}