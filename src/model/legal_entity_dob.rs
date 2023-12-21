use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegalEntityDob {
    ///The day of birth, between 1 and 31.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i64>,
    ///The month of birth, between 1 and 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    ///The four-digit year of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}
impl std::fmt::Display for LegalEntityDob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}