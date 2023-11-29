
use serde::{Serialize, Deserialize};
use super::{
    TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax,
    TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amusement_tax: Option<
        TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_lease_tax: Option<
        TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax,
    >,
    pub state: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}