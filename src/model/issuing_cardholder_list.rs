
use serde::{Serialize, Deserialize};
use super::IssuingCardholder;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingCardholderList {
    pub data: Vec<IssuingCardholder>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for IssuingCardholderList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}