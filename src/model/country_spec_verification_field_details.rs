use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountrySpecVerificationFieldDetails {
    ///Additional fields which are only required for some users.
    pub additional: Vec<String>,
    ///Fields which every account must eventually provide.
    pub minimum: Vec<String>,
}
impl std::fmt::Display for CountrySpecVerificationFieldDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}