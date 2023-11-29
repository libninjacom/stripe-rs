
use serde::{Serialize, Deserialize};
use super::IssuingDispute;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingDisputeList {
    pub data: Vec<IssuingDispute>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for IssuingDisputeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}