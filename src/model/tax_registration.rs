
use serde::{Serialize, Deserialize};
use super::TaxProductRegistrationsResourceCountryOptions;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRegistration {
    pub active_from: i64,
    pub country: String,
    pub country_options: TaxProductRegistrationsResourceCountryOptions,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub status: String,
}
impl std::fmt::Display for TaxRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}