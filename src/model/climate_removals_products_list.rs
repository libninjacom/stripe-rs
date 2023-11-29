
use serde::{Serialize, Deserialize};
use super::ClimateProduct;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClimateRemovalsProductsList {
    pub data: Vec<ClimateProduct>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for ClimateRemovalsProductsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}