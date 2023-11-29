
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountrySpecVerificationFieldDetails {
    pub additional: Vec<String>,
    pub minimum: Vec<String>,
}
impl std::fmt::Display for CountrySpecVerificationFieldDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}