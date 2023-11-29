
use serde::{Serialize, Deserialize};
use super::TaxRegistration;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductRegistrationsResourceTaxRegistrationList {
    pub data: Vec<TaxRegistration>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TaxProductRegistrationsResourceTaxRegistrationList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}