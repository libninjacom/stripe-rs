
use serde::{Serialize, Deserialize};
use super::VerificationDocumentSpecs;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VerificationSpecs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentSpecs>,
}
impl std::fmt::Display for VerificationSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}