use serde::{Serialize, Deserialize};
use super::TerminalReaderReaderResourceTippingConfig;
///Represents a per-transaction override of a reader configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalReaderReaderResourceProcessConfig {
    ///Override showing a tipping selection screen on this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    ///Represents a per-transaction tipping configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<TerminalReaderReaderResourceTippingConfig>,
}
impl std::fmt::Display for TerminalReaderReaderResourceProcessConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}