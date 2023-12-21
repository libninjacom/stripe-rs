use serde::{Serialize, Deserialize};
use super::TerminalConfigurationConfigurationResourceCurrencySpecificConfig;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalConfigurationConfigurationResourceTipping {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
}
impl std::fmt::Display for TerminalConfigurationConfigurationResourceTipping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}