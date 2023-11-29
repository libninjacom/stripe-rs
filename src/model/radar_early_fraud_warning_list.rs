
use serde::{Serialize, Deserialize};
use super::RadarEarlyFraudWarning;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RadarEarlyFraudWarningList {
    pub data: Vec<RadarEarlyFraudWarning>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for RadarEarlyFraudWarningList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}