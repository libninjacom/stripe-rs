use serde::{Serialize, Deserialize};
///Result from a document check
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GelatoDocumentReport {
    ///Address as it appears in the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<serde_json::Value>,
    ///Date of birth as it appears in the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<serde_json::Value>,
    ///Details on the verification error. Present when status is `unverified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    ///Expiration date of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<serde_json::Value>,
    ///Array of [File](https://stripe.com/docs/api/files) ids containing images for this document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    ///First name as it appears in the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    ///Issued date of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_date: Option<serde_json::Value>,
    ///Issuing country of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_country: Option<String>,
    ///Last name as it appears in the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    ///Document ID number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    ///Status of this `document` check.
    pub status: String,
    ///Type of the document.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for GelatoDocumentReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}