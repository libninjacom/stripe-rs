
use serde::{Serialize, Deserialize};
use super::{
    TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    TerminalConfigurationConfigurationResourceOfflineConfig,
    TerminalConfigurationConfigurationResourceTipping,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<
        TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    >,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_default: Option<bool>,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<TerminalConfigurationConfigurationResourceOfflineConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<TerminalConfigurationConfigurationResourceTipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<
        TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    >,
}
impl std::fmt::Display for TerminalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}