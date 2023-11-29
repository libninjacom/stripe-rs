
use serde::{Serialize, Deserialize};
use super::TerminalConfiguration;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalConfigurationConfigurationList {
    pub data: Vec<TerminalConfiguration>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TerminalConfigurationConfigurationList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}