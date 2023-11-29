
use serde::{Serialize, Deserialize};
use super::CountrySpecVerificationFields;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountrySpec {
    pub default_currency: String,
    pub id: String,
    pub object: String,
    pub supported_bank_account_currencies: serde_json::Value,
    pub supported_payment_currencies: Vec<String>,
    pub supported_payment_methods: Vec<String>,
    pub supported_transfer_countries: Vec<String>,
    pub verification_fields: CountrySpecVerificationFields,
}
impl std::fmt::Display for CountrySpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}