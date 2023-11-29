
use serde::{Serialize, Deserialize};
use super::TaxId;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxIDsList {
    pub data: Vec<TaxId>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TaxIDsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}