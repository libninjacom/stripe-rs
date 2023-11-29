
use serde::{Serialize, Deserialize};
use super::Address;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegalEntityCompany {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_license_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_purpose_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<serde_json::Value>,
}
impl std::fmt::Display for LegalEntityCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}