
use serde::{Serialize, Deserialize};
use super::Card;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardList {
    pub data: Vec<Card>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for CardList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}