
use serde::{Serialize, Deserialize};
use super::Topup;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TopupList {
    pub data: Vec<Topup>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TopupList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}