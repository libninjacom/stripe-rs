use serde::{Serialize, Deserialize};
///Represents a per-setup override of a reader configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalReaderReaderResourceProcessSetupConfig {}
impl std::fmt::Display for TerminalReaderReaderResourceProcessSetupConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}