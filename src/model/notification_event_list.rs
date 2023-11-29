
use serde::{Serialize, Deserialize};
use super::Event;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationEventList {
    pub data: Vec<Event>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for NotificationEventList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}