
use serde::{Serialize, Deserialize};
use super::{
    TaxProductResourceTaxSettingsDefaults, TaxProductResourceTaxSettingsStatusDetails,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxSettings {
    pub defaults: TaxProductResourceTaxSettingsDefaults,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_office: Option<serde_json::Value>,
    pub livemode: bool,
    pub object: String,
    pub status: String,
    pub status_details: TaxProductResourceTaxSettingsStatusDetails,
}
impl std::fmt::Display for TaxSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}