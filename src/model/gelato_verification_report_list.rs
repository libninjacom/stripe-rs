
use serde::{Serialize, Deserialize};
use super::IdentityVerificationReport;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GelatoVerificationReportList {
    pub data: Vec<IdentityVerificationReport>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for GelatoVerificationReportList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}