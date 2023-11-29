
use serde::{Serialize, Deserialize};
use super::TerminalReader;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalReaderRetrieveReader {
    pub data: Vec<TerminalReader>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TerminalReaderRetrieveReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}