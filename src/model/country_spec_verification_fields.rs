
use serde::{Serialize, Deserialize};
use super::CountrySpecVerificationFieldDetails;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountrySpecVerificationFields {
    pub company: CountrySpecVerificationFieldDetails,
    pub individual: CountrySpecVerificationFieldDetails,
}
impl std::fmt::Display for CountrySpecVerificationFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}