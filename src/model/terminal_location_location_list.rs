
use serde::{Serialize, Deserialize};
use super::TerminalLocation;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalLocationLocationList {
    pub data: Vec<TerminalLocation>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TerminalLocationLocationList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}