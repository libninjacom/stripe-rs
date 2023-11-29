
use serde::{Serialize, Deserialize};
use super::Address;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalLocation {
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<String>,
    pub display_name: String,
    pub id: String,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub object: String,
}
impl std::fmt::Display for TerminalLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}