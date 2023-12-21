use serde::{Serialize, Deserialize};
use super::{
    TaxProductResourceTaxSettingsStatusDetailsResourceActive,
    TaxProductResourceTaxSettingsStatusDetailsResourcePending,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductResourceTaxSettingsStatusDetails {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<TaxProductResourceTaxSettingsStatusDetailsResourceActive>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<TaxProductResourceTaxSettingsStatusDetailsResourcePending>,
}
impl std::fmt::Display for TaxProductResourceTaxSettingsStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}