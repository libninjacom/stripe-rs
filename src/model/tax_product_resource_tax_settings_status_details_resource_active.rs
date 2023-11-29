
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourceActive {}
impl std::fmt::Display for TaxProductResourceTaxSettingsStatusDetailsResourceActive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}