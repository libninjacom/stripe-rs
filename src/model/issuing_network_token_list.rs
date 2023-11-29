
use serde::{Serialize, Deserialize};
use super::IssuingToken;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingNetworkTokenList {
    pub data: Vec<IssuingToken>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for IssuingNetworkTokenList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}