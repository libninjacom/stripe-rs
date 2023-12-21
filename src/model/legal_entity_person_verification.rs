use serde::{Serialize, Deserialize};
use super::LegalEntityPersonVerificationDocument;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegalEntityPersonVerification {
    ///A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<serde_json::Value>,
    ///A user-displayable string describing the verification state for the person. For example, this may say "Provided identity information could not be verified".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    ///One of `document_address_mismatch`, `document_dob_mismatch`, `document_duplicate_type`, `document_id_number_mismatch`, `document_name_mismatch`, `document_nationality_mismatch`, `failed_keyed_identity`, or `failed_other`. A machine-readable code specifying the verification state for the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<LegalEntityPersonVerificationDocument>,
    ///The state of verification for the person. Possible values are `unverified`, `pending`, or `verified`.
    pub status: String,
}
impl std::fmt::Display for LegalEntityPersonVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}