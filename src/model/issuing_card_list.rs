
use serde::{Serialize, Deserialize};
use super::IssuingCard;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingCardList {
    pub data: Vec<IssuingCard>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for IssuingCardList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}