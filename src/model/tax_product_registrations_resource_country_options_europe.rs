
use serde::{Serialize, Deserialize};
use super::TaxProductRegistrationsResourceCountryOptionsEuStandard;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductRegistrationsResourceCountryOptionsEurope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<TaxProductRegistrationsResourceCountryOptionsEuStandard>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsEurope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}