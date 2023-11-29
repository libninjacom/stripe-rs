
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalConnectionToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub object: String,
    pub secret: String,
}
impl std::fmt::Display for TerminalConnectionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}