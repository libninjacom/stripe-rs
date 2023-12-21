use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductRegistrationsResourceCountryOptionsEuStandard {
    ///Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: String,
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsEuStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}