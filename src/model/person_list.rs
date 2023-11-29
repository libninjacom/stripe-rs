
use serde::{Serialize, Deserialize};
use super::Person;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonList {
    pub data: Vec<Person>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PersonList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}