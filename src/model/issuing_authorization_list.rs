
use serde::{Serialize, Deserialize};
use super::IssuingAuthorization;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingAuthorizationList {
    pub data: Vec<IssuingAuthorization>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for IssuingAuthorizationList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}