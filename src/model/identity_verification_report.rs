
use serde::{Serialize, Deserialize};
use super::{
    GelatoDocumentReport, GelatoIdNumberReport, GelatoSelfieReport,
    GelatoVerificationReportOptions,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationReport {
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<GelatoDocumentReport>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<GelatoIdNumberReport>,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GelatoVerificationReportOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<GelatoSelfieReport>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<String>,
}
impl std::fmt::Display for IdentityVerificationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}