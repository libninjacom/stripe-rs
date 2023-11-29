
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GelatoReportDocumentOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}
impl std::fmt::Display for GelatoReportDocumentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}