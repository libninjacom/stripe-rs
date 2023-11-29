
use serde::{Serialize, Deserialize};
use super::Price;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriceList {
    pub data: Vec<Price>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PriceList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}