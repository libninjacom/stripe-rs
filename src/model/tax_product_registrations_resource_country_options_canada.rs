use serde::{Serialize, Deserialize};
use super::TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductRegistrationsResourceCountryOptionsCanada {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_standard: Option<
        TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard,
    >,
    ///Type of registration in Canada.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsCanada {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}