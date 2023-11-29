
use serde::{Serialize, Deserialize};
use super::Capability;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListAccountCapability {
    pub data: Vec<Capability>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for ListAccountCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}