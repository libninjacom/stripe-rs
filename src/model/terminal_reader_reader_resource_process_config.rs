
use serde::{Serialize, Deserialize};
use super::TerminalReaderReaderResourceTippingConfig;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalReaderReaderResourceProcessConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<TerminalReaderReaderResourceTippingConfig>,
}
impl std::fmt::Display for TerminalReaderReaderResourceProcessConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}