
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<serde_json::Value>,
}
impl std::fmt::Display
for TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}