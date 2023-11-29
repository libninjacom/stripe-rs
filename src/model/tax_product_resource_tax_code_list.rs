
use serde::{Serialize, Deserialize};
use super::TaxCode;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductResourceTaxCodeList {
    pub data: Vec<TaxCode>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TaxProductResourceTaxCodeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}