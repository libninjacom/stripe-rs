use serde::{Serialize, Deserialize};
use super::{
    TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax,
    TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amusement_tax: Option<
        TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax,
    >,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_lease_tax: Option<
        TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax,
    >,
    ///Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: String,
    ///Type of registration in the US.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}