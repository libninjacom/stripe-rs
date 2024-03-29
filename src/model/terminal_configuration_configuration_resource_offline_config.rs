use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalConfigurationConfigurationResourceOfflineConfig {
    ///Determines whether to allow transactions to be collected while reader is offline. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl std::fmt::Display for TerminalConfigurationConfigurationResourceOfflineConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}