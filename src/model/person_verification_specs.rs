
use serde::{Serialize, Deserialize};
use super::PersonVerificationDocumentSpecs;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonVerificationSpecs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PersonVerificationDocumentSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentSpecs>,
}
impl std::fmt::Display for PersonVerificationSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}