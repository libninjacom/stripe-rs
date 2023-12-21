use serde::{Serialize, Deserialize};
use super::{GelatoReportDocumentOptions, GelatoReportIdNumberOptions};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GelatoVerificationReportOptions {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<GelatoReportDocumentOptions>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<GelatoReportIdNumberOptions>,
}
impl std::fmt::Display for GelatoVerificationReportOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}