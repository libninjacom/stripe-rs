
use serde::{Serialize, Deserialize};
use super::{
    Address, LegalEntityDob, LegalEntityPersonVerification,
    PersonAdditionalTosAcceptances, PersonRelationship,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Person {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tos_acceptances: Option<PersonAdditionalTosAcceptances>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<serde_json::Value>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<LegalEntityDob>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<PersonRelationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<serde_json::Value>,
    #[serde(rename = "ssn_last_4_provided")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last4_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<LegalEntityPersonVerification>,
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}