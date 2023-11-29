
use serde::{Serialize, Deserialize};
use super::TaxRate;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxRatesList {
    pub data: Vec<TaxRate>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TaxRatesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}