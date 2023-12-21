use serde::{Serialize, Deserialize};
use super::{
    TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    TerminalConfigurationConfigurationResourceOfflineConfig,
    TerminalConfigurationConfigurationResourceTipping,
};
///A Configurations object represents how features should be configured for terminal readers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalConfiguration {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<
        TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    >,
    ///Unique identifier for the object.
    pub id: String,
    ///Whether this Configuration is the default for your account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_default: Option<bool>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<TerminalConfigurationConfigurationResourceOfflineConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<TerminalConfigurationConfigurationResourceTipping>,
    ///
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